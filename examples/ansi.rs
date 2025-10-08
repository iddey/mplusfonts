use std::convert::Infallible;
use std::sync::LazyLock;

use embedded_graphics::pixelcolor::{Gray4, Rgb565};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorEvent::{KeyDown, Quit};
use embedded_graphics_simulator::sdl2::Keycode;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_text::TextBox;
use embedded_text::plugin::ansi::Ansi;
use embedded_text::style::TextBoxStyle;
use mplusfonts::style::BitmapFontStyleBuilder;
use mplusfonts::{BitmapFont, mplus};

const RESET_ALL: &str = "\x1B[0m";
const UL: &str = "\x1B[4m";
const ST: &str = "\x1B[9m";
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";
const PURPLE: &str = "\x1B[38;2;80;0;80m";
const RESET: &str = "\x1B[39m";
const BG_BLACK: &str = "\x1B[40m";
const BG_DARK: &str = "\x1B[48;2;80;80;80m";
const BG_LIGHT: &str = "\x1B[48;2;216;216;216m";
const BG_MEDIUM: &str = "\x1B[48;5;248m";
const BG_RESET: &str = "\x1B[49m";
const BG_100: &str = "\x1B[100m";

#[mplusfonts::strings]
static TEXTUAL_DATA: LazyLock<(String, [BitmapFont<'static, Gray4, 4>; 3])> = LazyLock::new(|| {
    let text = format!(
        "{RED}\u{25BC}{BG_LIGHT}\u{A0}{GREEN}\u{25B2}{BG_MEDIUM}\u{A0}{BLUE}A{RESET}very{BG_RESET} \
        {YELLOW}H{RESET}{BG_DARK}ill{BG_BLACK} {RED}{ST}is{RESET_ALL}{GREEN}was{RESET} technically \
        a place you may have{BG_DARK} {BG_100}{PURPLE}{UL}s{BG_DARK}e{BG_100}e{BG_DARK}n{BG_RESET} \
        here{RESET_ALL}.\t{RESET}1 2 3 4 5 6 7 8 9 0\t{RED}赤{GREEN}緑{BLUE}青{YELLOW}黄{PURPLE}紫 \
        {RESET}stands for {BG_DARK}{RED}\u{25A0}{GREEN}\u{25A0}{BLUE}\u{25A0}{YELLOW}\u{25A0}{PURPL\
        E}\u{25A0}{RESET}{BG_RESET}",
    );

    let bitmap_fonts = [
        #[strings::emit]
        mplus!(2, 480, line_height(18), true, 4, 4),
        #[strings::emit]
        mplus!(code(100), 575, code_line_height(18), true, 4, 4),
        #[strings::emit]
        mplus!(code(125), 575, code_line_height(18), true, 4, 4),
    ];

    (text, bitmap_fonts)
});

/// Displays how ANSI escape codes can be used, dynamically changing the text color and decorations.
pub fn main() -> Result<(), Infallible> {
    let mut display_rgb: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));
    let mut display_gray: SimulatorDisplay<Gray4> = SimulatorDisplay::new(Size::new(240, 240));

    let output_settings = OutputSettingsBuilder::new()
        .scale(3)
        .pixel_spacing(1)
        .build();

    let mut window = Window::new("Simulator", &output_settings);

    let mut display_color = true;

    'running: loop {
        if display_color {
            draw_rgb(&mut display_rgb)?;

            window.update(&display_rgb);
        } else {
            draw_gray(&mut display_gray)?;

            window.update(&display_gray);
        }

        'waiting: loop {
            for event in window.events() {
                match event {
                    KeyDown {
                        keycode: Keycode::SPACE | Keycode::RETURN,
                        ..
                    } => break 'waiting,
                    Quit => break 'running Ok(()),
                    _ => continue 'waiting,
                }
            }
        }

        display_color = !display_color;
    }
}

macro_rules! impl_draw {
    ($fn_ident:ident, $color_type:ty) => {
        fn $fn_ident<D: DrawTarget<Color = $color_type>>(target: &mut D) -> Result<(), D::Error> {
            let (ref text, ref bitmap_fonts) = *TEXTUAL_DATA;

            for (font, y) in bitmap_fonts.into_iter().zip([6, 84, 162]) {
                let textbox = TextBox::with_textbox_style(
                    text,
                    Rectangle::new(Point::new(8, y), Size::new(224, 72)),
                    BitmapFontStyleBuilder::new().font(font).build(),
                    TextBoxStyle::default(),
                );

                let textbox = textbox.add_plugin(Ansi::new());

                textbox.draw(target)?;
            }

            Ok(())
        }
    };
}

impl_draw!(draw_rgb, Rgb565);
impl_draw!(draw_gray, Gray4);
