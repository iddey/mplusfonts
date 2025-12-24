//! Miniature board with a built-in monochrome display and an RP2040
//! Featuring the typical two-megabyte flash memory

#![no_std]
#![no_main]

mod rgb_led;

use core::panic::PanicInfo;
use core::ptr::addr_of_mut;

use embassy_executor::Executor;
use embassy_rp::bind_interrupts;
use embassy_rp::multicore::{Stack, spawn_core1};
use embassy_rp::peripherals::{I2C1, PIO0, USB};
use embassy_rp::{i2c, pio, usb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::lazy_lock::LazyLock;
use embassy_sync::pubsub;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::{Baseline, Text};
use heapless::Vec;
use mplusfonts::style::BitmapFontStyle;
use mplusfonts::{BitmapFont, mplus};
use mplusfonts_examples_common::BITMAP_FONT_1;
use mplusfonts_examples_common::{display, usb_serial};
use ssd1306::Ssd1306;
use ssd1306::command::AddrMode;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::I2CInterface as I2cInterface;
use ssd1306::prelude::*;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => pio::InterruptHandler<PIO0>;
    I2C1_IRQ => i2c::InterruptHandler<I2C1>;
});

// Buffer length: USB control request data
const N: usize = 64;

type UsbDriver = usb::Driver<'static, USB>;
type I2cDriver = i2c::I2c<'static, I2C1, i2c::Blocking>;
type PubSubChannel = pubsub::PubSubChannel<CriticalSectionRawMutex, Vec<u8, N>, 1, 3, 1>;

static mut CORE1_STACK: Stack<8192> = Stack::new();
static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();
static USB_CONTROL: StaticCell<[u8; N]> = StaticCell::new();
static CDC_ACM_STATE: StaticCell<State> = StaticCell::new();
static PUB_SUB_CHANNEL: StaticCell<PubSubChannel> = StaticCell::new();

#[mplusfonts::strings]
static TEXTUAL_DATA: LazyLock<(&str, BitmapFont<'static, BinaryColor, 2>)> = LazyLock::new(|| {
    let text = "Listening...";

    #[strings::emit]
    let bitmap_font = mplus!(1, 480, line_height(15), true, 2, 1);

    (text, bitmap_font)
});

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rgb_led::red_halt()
}

#[embassy_executor::task]
async fn rgb_led_task() {
    rgb_led::blink().await
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, UsbDriver>) -> ! {
    device.run().await
}

#[embassy_executor::task]
async fn receive_task(mut host_reader: usb_serial::UsbHostReader<'static, UsbDriver, N>) -> ! {
    host_reader.run().await
}

#[embassy_executor::task]
async fn do_echo_task(mut host_writer: usb_serial::UsbHostWriter<'static, UsbDriver, N>) -> ! {
    host_writer.run().await
}

// OLED screen uses one bit per pixel
type D = Ssd1306<I2cInterface<I2cDriver>, DisplaySize72x40, BufferedGraphicsMode<DisplaySize72x40>>;
type T = BitmapFontStyle<'static, 'static, BinaryColor, BinaryColor, 1>;
type M = CriticalSectionRawMutex;

#[embassy_executor::task]
async fn display_task(mut text_writer: display::TextWriter<'static, D, T, M, N, 1, 3, 1>) -> ! {
    text_writer.run().await
}

#[embassy_executor::task]
async fn relay_task(mut unit_writer: rgb_led::UnitWriter<'static, Vec<u8, N>>) -> ! {
    unit_writer.run().await
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // Peripherals of RP2040
    let p = embassy_rp::init(Default::default());

    // Set up WS2812 as an activity and panic indicator
    let mut pio = pio::Pio::new(p.PIO0, Irqs);
    let ws2812 = rgb_led::Ws2812::new(&mut pio.common, pio.sm0, p.DMA_CH0, p.PIN_12);
    rgb_led::init(ws2812);

    // Serial over USB; this device will be an endpoint
    let driver = usb::Driver::new(p.USB, Irqs);
    let config = usb_serial::config("01Space", "RP2040-0.42LCD", "0123456789");
    let control_buf = USB_CONTROL.init([0; _]);
    let mut builder = usb_serial::builder(driver, config, control_buf);
    let state = CDC_ACM_STATE.init(State::new());
    let max_packet_size = config.max_packet_size_0.into();
    let class = CdcAcmClass::new(&mut builder, state, max_packet_size);
    let (sender, receiver) = class.split();
    let device = builder.build();

    // Make an echo, but drive packets through a channel first
    let channel = PUB_SUB_CHANNEL.init(PubSubChannel::new());
    let publisher = channel.dyn_publisher().unwrap();
    let subscriber = channel.dyn_subscriber().unwrap();
    let host_reader = usb_serial::UsbHostReader::new(receiver, publisher);
    let host_writer = usb_serial::UsbHostWriter::new(sender, subscriber);

    // Signal the RGB LED indicator
    let subscriber = channel.dyn_subscriber().unwrap();
    let unit_writer = rgb_led::UnitWriter::new(subscriber);

    // Utilize the second CPU core for graphics
    let subscriber = channel.subscriber().unwrap();
    spawn_core1(
        p.CORE1,
        unsafe { &mut *addr_of_mut!(CORE1_STACK) },
        move || {
            // Crank it up to 800 kilohertz
            let mut config = i2c::Config::default();
            config.frequency = 800_000;

            // No shared bus with this device
            let i2c = i2c::I2c::new_blocking(p.I2C1, p.PIN_23, p.PIN_22, config);
            let interface = I2cInterface::new(i2c, 0x3C, 0x40);
            let ssd1306 = Ssd1306::new(interface, DisplaySize72x40, DisplayRotation::Rotate180);
            let mut ssd1306 = ssd1306.into_buffered_graphics_mode();
            ssd1306.init_with_addr_mode(AddrMode::Horizontal).unwrap();

            // Draw a status bar
            let (text, bitmap_font) = TEXTUAL_DATA.get();
            let character_style = BitmapFontStyle::new(bitmap_font, BinaryColor::On);
            let text = Text::with_baseline(text, Point::zero(), character_style, Baseline::Top);
            let text_area = Rectangle {
                top_left: Point::new(0, 25),
                size: ssd1306.bounding_box().size - Size::new(0, 25),
            };
            let next_position = text.draw(&mut ssd1306.cropped(&text_area)).unwrap();
            let text_layout_width = next_position.x.try_into().unwrap();
            let fill_area = Rectangle {
                top_left: Point::new(next_position.x, 25),
                size: ssd1306.bounding_box().size - Size::new(text_layout_width, 25),
            };
            ssd1306.fill_solid(&fill_area, BinaryColor::Off).unwrap();
            ssd1306.flush().unwrap();

            // Tap into pub-sub channel for on-screen text rendering
            let character_style = BitmapFontStyle::new(&BITMAP_FONT_1, BinaryColor::On);
            let width = ssd1306.bounding_box().size.width;
            let mut text_writer = display::TextWriter {
                target: ssd1306,
                position: Point::zero(),
                max_size: Size::new(width, 25),
                renderer: character_style,
                baseline: Baseline::Top,
                bg_color: BinaryColor::Off,
                flush_fn: Ssd1306::flush,
                subscriber,
            };
            text_writer.clear();

            let executor = EXECUTOR1.init(Executor::new());
            executor.run(|spawner| {
                spawner.spawn(display_task(text_writer)).unwrap();
            })
        },
    );

    let executor = EXECUTOR0.init(Executor::new());
    executor.run(|spawner| {
        spawner.spawn(rgb_led_task()).unwrap();
        spawner.spawn(usb_task(device)).unwrap();
        spawner.spawn(receive_task(host_reader)).unwrap();
        spawner.spawn(do_echo_task(host_writer)).unwrap();
        spawner.spawn(relay_task(unit_writer)).unwrap();
    })
}
