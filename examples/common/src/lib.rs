#![no_std]

pub mod usb_serial;

use embedded_graphics::pixelcolor::BinaryColor;
use mplusfonts::{BitmapFont, mplus};

pub static BITMAP_FONT_1: BitmapFont<BinaryColor, 1> =
    mplus!(2, 525, line_height(25), true, 1, 1, ..);
