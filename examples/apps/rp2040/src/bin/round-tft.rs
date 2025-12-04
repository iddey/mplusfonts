//! Watch-sized board with a built-in color display and an RP2040
//! Featuring the typical two-megabyte flash memory

#![no_std]
#![no_main]

use core::cell::RefCell;
use core::ptr::addr_of_mut;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Executor;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::multicore::{Stack, spawn_core1};
use embassy_rp::peripherals::SPI1;
use embassy_rp::spi;
use embassy_sync::blocking_mutex::NoopMutex;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::pubsub;
use embassy_sync::pubsub::WaitResult::Message;
use embassy_time::Delay;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use heapless::String;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::GC9A01;
use mipidsi::options::{ColorInversion, ColorOrder, Orientation, Rotation};
use mplusfonts::mplus;
use mplusfonts::style::BitmapFontStyleBuilder;
use mplusfonts_examples_common::signals;
use panic_halt as _;
use static_cell::StaticCell;

type SpiDriver = spi::Spi<'static, SPI1, spi::Blocking>;
type SpiBusDevice = SpiDeviceWithConfig<'static, NoopRawMutex, SpiDriver, Output<'static>>;
type Display<'a> = mipidsi::Display<SpiInterface<'a, SpiBusDevice, Output<'a>>, GC9A01, Output<'a>>;
type PubSubChannel = pubsub::PubSubChannel<CriticalSectionRawMutex, String<8>, 1, 1, 1>;
type Subscriber = pubsub::Subscriber<'static, CriticalSectionRawMutex, String<8>, 1, 1, 1>;

static mut CORE1_STACK: Stack<8192> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static SPI_BUS: StaticCell<NoopMutex<RefCell<SpiDriver>>> = StaticCell::new();
static LCD_BUFFER: StaticCell<[u8; 512]> = StaticCell::new();
static PUB_SUB_CHANNEL: StaticCell<PubSubChannel> = StaticCell::new();

#[embassy_executor::task]
async fn time_task(mut clock: signals::Clock<'static>) -> ! {
    clock.run().await
}

#[embassy_executor::task]
async fn display_task(mut display: Display<'static>, mut subscriber: Subscriber) -> ! {
    let bitmap_font = mplus!(1, 500, 50, true, 1, 4, '0'..='9', [":"]);

    let character_style = BitmapFontStyleBuilder::new()
        .text_color(Rgb565::new(30, 60, 30))
        .font(&bitmap_font)
        .build();

    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    loop {
        let result = subscriber.next_message().await;

        let Message(digits) = result else {
            continue;
        };

        let text = Text::with_text_style(
            &digits,
            Point::new(120, 120),
            character_style.clone(),
            text_style,
        );

        text.draw(&mut display).unwrap();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // Peripherals of RP2040
    let p = embassy_rp::init(Default::default());

    // Shared SPI bus
    let spi = spi::Spi::new_blocking_txonly(p.SPI1, p.PIN_10, p.PIN_11, Default::default());
    let spi_bus = SPI_BUS.init(NoopMutex::new(RefCell::new(spi)));

    // Generate ticks
    let channel = PUB_SUB_CHANNEL.init(PubSubChannel::new());
    let publisher = channel.dyn_publisher().unwrap();
    let clock = signals::Clock::new(12, 0, 0, publisher);

    // Utilize the second CPU core for graphics
    let subscriber = channel.subscriber().unwrap();
    spawn_core1(
        p.CORE1,
        unsafe { &mut *addr_of_mut!(CORE1_STACK) },
        move || {
            // Diminishing returns over 16 MHz
            let mut config = spi::Config::default();
            config.frequency = 16_000_000;

            // Round TFT-LCD on shared SPI bus
            let buffer = LCD_BUFFER.init([0; _]);
            let cs = Output::new(p.PIN_9, Level::High);
            let dc = Output::new(p.PIN_8, Level::Low);
            let rst = Output::new(p.PIN_12, Level::High);
            let _blk = Output::new(p.PIN_25, Level::High);
            let spi_device = SpiDeviceWithConfig::new(spi_bus, cs, config);
            let interface = SpiInterface::new(spi_device, dc, buffer);
            let mut display = Builder::new(GC9A01, interface)
                .reset_pin(rst)
                .color_order(ColorOrder::Bgr)
                .invert_colors(ColorInversion::Inverted)
                .orientation(Orientation::new().rotate(Rotation::Deg90))
                .init(&mut Delay)
                .unwrap();

            display
                .fill_solid(&display.bounding_box(), Rgb565::BLACK)
                .unwrap();

            let executor = EXECUTOR1.init(Executor::new());
            executor.run(|spawner| {
                spawner.spawn(display_task(display, subscriber)).unwrap();
            })
        },
    );

    let executor = EXECUTOR0.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(time_task(clock)).unwrap();
    })
}
