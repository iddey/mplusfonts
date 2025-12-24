mod reader;
mod writer;

use embassy_usb::driver::Driver;
use embassy_usb::{Builder, Config};
use static_cell::StaticCell;

pub use reader::*;
pub use writer::*;

const VID: u16 = 0x5A5A;
const PID: u16 = 0xA5A5;

static CONFIG_DESCRIPTOR: StaticCell<[u8; 128]> = StaticCell::new();
static BOS_DESCRIPTOR: StaticCell<[u8; 16]> = StaticCell::new();
static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();

pub fn config<'a>(manufacturer: &'a str, product: &'a str, serial_number: &'a str) -> Config<'a> {
    let mut config = Config::new(VID, PID);
    config.manufacturer = Some(manufacturer);
    config.product = Some(product);
    config.serial_number = Some(serial_number);

    config
}

pub fn builder<'d, D: Driver<'d>>(
    driver: D,
    config: Config<'d>,
    control_buf: &'d mut [u8],
) -> Builder<'d, D> {
    let config_descriptor_buf = CONFIG_DESCRIPTOR.init([0; _]);
    let bos_descriptor_buf = BOS_DESCRIPTOR.init([0; _]);
    let msos_descriptor_buf = MSOS_DESCRIPTOR.init([0; _]);

    Builder::new(
        driver,
        config,
        config_descriptor_buf,
        bos_descriptor_buf,
        msos_descriptor_buf,
        control_buf,
    )
}
