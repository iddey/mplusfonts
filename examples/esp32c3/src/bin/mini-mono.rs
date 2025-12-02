//! Miniature board with a built-in monochrome display and an ESP32-C3
//! Featuring the typical four-megabyte flash memory

#![no_std]
#![no_main]

mod display;
mod rgb_led;
mod usb_serial_jtag;

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::lazy_lock::LazyLock;
use embassy_sync::pubsub;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::text::{Baseline, Text};
use esp_hal::Blocking;
use esp_hal::interrupt::software::SoftwareInterruptControl;
use esp_hal::time::Rate;
use esp_hal::timer::timg;
use esp_hal::usb_serial_jtag::UsbSerialJtag;
use esp_hal::{i2c, rmt};
use esp_hal_smartled::{SmartLedsAdapter, smart_led_buffer};
use heapless::Vec;
use mplusfonts::style::BitmapFontStyle;
use mplusfonts::{BitmapFont, mplus};
use mplusfonts_examples_common::BITMAP_FONT_1;
use ssd1306::Ssd1306;
use ssd1306::command::AddrMode;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::I2CInterface as I2cInterface;
use ssd1306::prelude::*;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

// Buffer length: USB control endpoint; up to 64-byte data payload size
const N: usize = 64;

type I2cDriver = i2c::master::I2c<'static, Blocking>;
type PubSubChannel = pubsub::PubSubChannel<CriticalSectionRawMutex, Vec<u8, N>, 1, 3, 1>;

static PUB_SUB_CHANNEL: StaticCell<PubSubChannel> = StaticCell::new();
static SMART_LED_BUFFER: StaticCell<[rmt::PulseCode; 25]> = StaticCell::new();

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
async fn receive_task(mut host_reader: usb_serial_jtag::UsbHostReader<'static, N>) -> ! {
    host_reader.run().await
}

#[embassy_executor::task]
async fn do_echo_task(mut host_writer: usb_serial_jtag::UsbHostWriter<'static, N>) -> ! {
    host_writer.run().await
}

// OLED screen uses one bit per pixel
type D = Ssd1306<I2CInterface<I2cDriver>, DisplaySize72x40, BufferedGraphicsMode<DisplaySize72x40>>;
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

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // Peripherals of ESP32
    let p = esp_hal::init(Default::default());

    // When idle, this core will wait for an interrupt
    let timg0 = timg::TimerGroup::new(p.TIMG0);
    let control = SoftwareInterruptControl::new(p.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, control.software_interrupt0);

    // Set up WS2812 as an activity and panic indicator
    let rmt = rmt::Rmt::new(p.RMT, Rate::from_mhz(80)).unwrap();
    let buffer = SMART_LED_BUFFER.init(smart_led_buffer!(1));
    let adapter = SmartLedsAdapter::new(rmt.channel0, p.GPIO2, buffer);
    rgb_led::init(adapter);

    // Serial over USB, which is hardwired as a CDC-ACM device
    let (receiver, sender) = UsbSerialJtag::new(p.USB_DEVICE).into_async().split();

    // Make an echo, but drive packets through a channel first
    let channel = PUB_SUB_CHANNEL.init(PubSubChannel::new());
    let publisher = channel.dyn_publisher().unwrap();
    let subscriber = channel.dyn_subscriber().unwrap();
    let host_reader = usb_serial_jtag::UsbHostReader::new(receiver, publisher);
    let host_writer = usb_serial_jtag::UsbHostWriter::new(sender, subscriber);

    // Signal the RGB LED indicator
    let subscriber = channel.dyn_subscriber().unwrap();
    let unit_writer = rgb_led::UnitWriter::new(subscriber);

    // Notice the second CPU core that is absent in this example
    let subscriber = channel.subscriber().unwrap();

    // Crank it up to 800 kilohertz
    let config = i2c::master::Config::default().with_frequency(Rate::from_khz(800));

    // No shared bus with this device
    let i2c = i2c::master::I2c::new(p.I2C0, config);
    let i2c = i2c.unwrap().with_sda(p.GPIO5).with_scl(p.GPIO6);
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

    // Thread-mode executor on the main thread
    spawner.spawn(rgb_led_task()).unwrap();
    spawner.spawn(receive_task(host_reader)).unwrap();
    spawner.spawn(do_echo_task(host_writer)).unwrap();
    spawner.spawn(display_task(text_writer)).unwrap();
    spawner.spawn(relay_task(unit_writer)).unwrap();
}
