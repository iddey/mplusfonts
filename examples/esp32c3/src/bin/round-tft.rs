//! Watch-sized board with a built-in color display and an ESP32-C3
//! Featuring the typical four-megabyte flash memory

#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::NoopMutex;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::pubsub;
use embassy_sync::pubsub::WaitResult::Message;
use embassy_time::Delay;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Baseline, Text, TextStyleBuilder};
use esp_hal::Blocking;
use esp_hal::gpio::{Level, Output};
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::spi;
use esp_hal::time::Rate;
use esp_hal::timer::timg;
use heapless::String;
use mipidsi::interface::SpiInterface;
use mipidsi::models::GC9A01;
use mipidsi::options::{ColorInversion, ColorOrder, Orientation, Rotation};
use mipidsi::{Builder, NoResetPin};
use mplusfonts::mplus;
use mplusfonts::style::BitmapFontStyleBuilder;
use mplusfonts_examples_common::signals;
use panic_halt as _;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

type SpiDriver = spi::master::Spi<'static, Blocking>;
type SpiBusDevice = SpiDeviceWithConfig<'static, NoopRawMutex, SpiDriver, Output<'static>>;
type Display<'a> = mipidsi::Display<SpiInterface<'a, SpiBusDevice, Output<'a>>, GC9A01, NoResetPin>;
type PubSubChannel = pubsub::PubSubChannel<CriticalSectionRawMutex, String<8>, 1, 1, 1>;
type Subscriber = pubsub::Subscriber<'static, CriticalSectionRawMutex, String<8>, 1, 1, 1>;

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

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Peripherals of ESP32
    let p = esp_hal::init(Default::default());

    // When idle, this core will wait for an interrupt
    let timg0 = timg::TimerGroup::new(p.TIMG0);
    let control = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, control.software_interrupt0);

    // Shared SPI bus
    let spi = spi::master::Spi::new(p.SPI2, Default::default());
    let spi = spi.unwrap().with_sck(p.GPIO6).with_mosi(p.GPIO7);
    let spi_bus = SPI_BUS.init(NoopMutex::new(RefCell::new(spi)));

    // Generate ticks
    let channel = PUB_SUB_CHANNEL.init(PubSubChannel::new());
    let publisher = channel.dyn_publisher().unwrap();
    let clock = signals::Clock::new(12, 0, 0, publisher);

    // Notice the second CPU core that is absent in this example
    let subscriber = channel.subscriber().unwrap();

    // Diminishing returns over 16 MHz
    let config = spi::master::Config::default().with_frequency(Rate::from_mhz(16));

    // Round TFT-LCD on shared SPI bus
    let buffer = LCD_BUFFER.init([0; _]);
    let cs = Output::new(p.GPIO10, Level::High, Default::default());
    let dc = Output::new(p.GPIO2, Level::Low, Default::default());
    let _blk = Output::new(p.GPIO3, Level::High, Default::default());
    let spi_device = SpiDeviceWithConfig::new(spi_bus, cs, config);
    let interface = SpiInterface::new(spi_device, dc, buffer);
    let mut display = Builder::new(GC9A01, interface)
        .color_order(ColorOrder::Bgr)
        .invert_colors(ColorInversion::Inverted)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut Delay)
        .unwrap();

    display
        .fill_solid(&display.bounding_box(), Rgb565::BLACK)
        .unwrap();

    spawner.spawn(time_task(clock)).unwrap();
    spawner.spawn(display_task(display, subscriber)).unwrap();
}
