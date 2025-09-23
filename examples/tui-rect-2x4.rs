use core::convert::Infallible;
use core::iter;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics_simulator::SimulatorEvent::{KeyDown, Quit};
use embedded_graphics_simulator::sdl2::{Keycode, Mod};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_text::TextBox;
use embedded_text::alignment::HorizontalAlignment;
use embedded_text::style::TextBoxStyle;
use mplusfonts::mplus;
use mplusfonts::style::BitmapFontStyleBuilder;

/// Displays all 230 2-by-4 block mosaic characters (plus the ` `, `▌`, `▐`, and `█` characters,
/// the `🮂`, `▀`, `🮅`, `▆`, `▄`, and `▂` characters, the `𜺨`, `▘`, `🯦`, `▖`, and `𜺣` characters,
/// the `▛`, `▜`, `▞`, `▚`, `▟`, and `▙` characters, the `𜺫`, `▝`, `🯧`, `▗`, and `𜺠` characters).
#[mplusfonts::strings]
pub fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));

    #[strings::emit]
    let bitmap_font = mplus!(code(100), 450, code_line_height(27), true, 1, 4);

    let builder = BitmapFontStyleBuilder::new().text_color(Rgb565::new(18, 56, 20));

    let text_rows = [
        "\u{A0} 0 1 2 3 4 5 6 7",
        "0   𜺨 𜺫 🮂 𜴀 ▘ 𜴁 𜴂",
        "1 𜴉 𜴊 𜴋 𜴌 🯦 𜴍 𜴎 𜴏",
        "2 𜴘 𜴙 𜴚 𜴛 𜴜 𜴝 𜴞 𜴟",
        "3 𜴧 𜴨 𜴩 𜴪 𜴫 𜴬 𜴭 𜴮",
        "\u{A0}               \u{A0}",
        "\u{A0} 8 9 A B C D E F",
        "0 𜴃 𜴄 ▝ 𜴅 𜴆 𜴇 𜴈 ▀",
        "1 𜴐 𜴑 𜴒 𜴓 𜴔 𜴕 𜴖 𜴗",
        "2 🯧 𜴠 𜴡 𜴢 𜴣 𜴤 𜴥 𜴦",
        "3 𜴯 𜴰 𜴱 𜴲 𜴳 𜴴 𜴵 🮅",
        "\u{A0}               \u{A0}",
        "\u{A0} 0 1 2 3 4 5 6 7",
        "4 𜺣 𜴶 𜴷 𜴸 𜴹 𜴺 𜴻 𜴼",
        "5 ▖ 𜵅 𜵆 𜵇 𜵈 ▌ 𜵉 𜵊",
        "6 𜵑 𜵒 𜵓 𜵔 𜵕 𜵖 𜵗 𜵘",
        "7 𜵡 𜵢 𜵣 𜵤 𜵥 𜵦 𜵧 𜵨",
        "\u{A0}               \u{A0}",
        "\u{A0} 8 9 A B C D E F",
        "4 𜴽 𜴾 𜴿 𜵀 𜵁 𜵂 𜵃 𜵄",
        "5 𜵋 𜵌 ▞ 𜵍 𜵎 𜵏 𜵐 ▛",
        "6 𜵙 𜵚 𜵛 𜵜 𜵝 𜵞 𜵟 𜵠",
        "7 𜵩 𜵪 𜵫 𜵬 𜵭 𜵮 𜵯 𜵰",
        "\u{A0}               \u{A0}",
        "\u{A0} 0 1 2 3 4 5 6 7",
        "8 𜺠 𜵱 𜵲 𜵳 𜵴 𜵵 𜵶 𜵷",
        "9 𜶀 𜶁 𜶂 𜶃 𜶄 𜶅 𜶆 𜶇",
        "A ▗ 𜶐 𜶑 𜶒 𜶓 ▚ 𜶔 𜶕",
        "B 𜶜 𜶝 𜶞 𜶟 𜶠 𜶡 𜶢 𜶣",
        "\u{A0}               \u{A0}",
        "\u{A0} 8 9 A B C D E F",
        "8 𜵸 𜵹 𜵺 𜵻 𜵼 𜵽 𜵾 𜵿",
        "9 𜶈 𜶉 𜶊 𜶋 𜶌 𜶍 𜶎 𜶏",
        "A 𜶖 𜶗 ▐ 𜶘 𜶙 𜶚 𜶛 ▜",
        "B 𜶤 𜶥 𜶦 𜶧 𜶨 𜶩 𜶪 𜶫",
        "\u{A0}               \u{A0}",
        "\u{A0} 0 1 2 3 4 5 6 7",
        "C ▂ 𜶬 𜶭 𜶮 𜶯 𜶰 𜶱 𜶲",
        "D 𜶻 𜶼 𜶽 𜶾 𜶿 𜷀 𜷁 𜷂",
        "E 𜷋 𜷌 𜷍 𜷎 𜷏 𜷐 𜷑 𜷒",
        "F ▄ 𜷛 𜷜 𜷝 𜷞 ▙ 𜷟 𜷠",
        "\u{A0}               \u{A0}",
        "\u{A0} 8 9 A B C D E F",
        "C 𜶳 𜶴 𜶵 𜶶 𜶷 𜶸 𜶹 𜶺",
        "D 𜷃 𜷄 𜷅 𜷆 𜷇 𜷈 𜷉 𜷊",
        "E 𜷓 𜷔 𜷕 𜷖 𜷗 𜷘 𜷙 𜷚",
        "F 𜷡 𜷢 ▟ 𜷣 ▆ 𜷤 𜷥 █",
        "\u{A0}               \u{A0}",
    ];

    let output_settings = OutputSettingsBuilder::new()
        .scale(3)
        .pixel_spacing(1)
        .build();

    #[strings::skip]
    let mut window = Window::new("Simulator", &output_settings);

    let mut row_offset = 0;

    'running: loop {
        let text = String::from_iter(
            text_rows
                .into_iter()
                .cycle()
                .skip(row_offset)
                .take(8)
                .zip(iter::repeat("\n"))
                .map(|(string, newline)| [string, newline])
                .flatten(),
        );

        let textbox = TextBox::with_textbox_style(
            &text,
            Rectangle::new(Point::new(10, 12), Size::new(220, 216)),
            builder.clone().font(&bitmap_font).build(),
            TextBoxStyle::with_alignment(HorizontalAlignment::Center),
        );

        textbox.draw(&mut display)?;

        window.update(&display);

        let increment = 'waiting: loop {
            for event in window.events() {
                match event {
                    KeyDown {
                        keycode: Keycode::SPACE | Keycode::DOWN,
                        ..
                    } => break 'waiting 1,
                    KeyDown {
                        keycode: Keycode::BACKSPACE | Keycode::UP,
                        ..
                    } => break 'waiting text_rows.len() - 1,
                    KeyDown {
                        keycode: Keycode::PAGEDOWN,
                        keymod,
                        ..
                    } => {
                        if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                            break 'waiting 12;
                        } else {
                            break 'waiting 6;
                        }
                    }
                    KeyDown {
                        keycode: Keycode::PAGEUP,
                        keymod,
                        ..
                    } => {
                        if keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD) {
                            break 'waiting text_rows.len() - 12;
                        } else {
                            break 'waiting text_rows.len() - 6;
                        }
                    }
                    Quit => break 'running Ok(()),
                    _ => continue 'waiting,
                }
            }
        };

        row_offset = row_offset.wrapping_add(increment) % text_rows.len();
    }
}
