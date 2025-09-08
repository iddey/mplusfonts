mod chain;
mod cross;
mod lines;

use swash::zeno::{Placement, Vector};

use super::GlyphMetrics;

pub use chain::{Chain, ChainList};

mod base_id {
    pub const BOX_DRAWING: u16 = 1024;
}

type Image = (Vec<u8>, Placement);
type ImageCluster = Vec<Image>;
type Render = fn(&GlyphMetrics, Vector) -> ImageCluster;

pub struct GlyphDrawing {
    pub id: u16,
    pub key: char,
    pub render: Render,
}

impl GlyphDrawing {
    pub fn get(key: char) -> Option<Self> {
        match key {
            ..='\u{24FF}' => None,
            '\u{2500}'..='\u{257F}' | '\u{1FBAF}' => Self::try_box_drawing_components(key),
            '\u{2580}'..='\u{D7FF}' | '\u{E000}'.. => None,
        }
    }

    fn try_box_drawing_components(key: char) -> Option<Self> {
        let result: Option<(_, Render)> = match key {
            ..='\u{24FF}' => None,
            '\u{2500}' => Some((0x5F, lines::light_horizontal)),
            '\u{2501}' => Some((0x2F, lines::heavy_horizontal)),
            '\u{2502}' => Some((0x72, lines::light_vertical)),
            '\u{2503}' => Some((0x41, lines::heavy_vertical)),
            '\u{2504}' => Some((0x67, lines::light_triple_dash_horizontal)),
            '\u{2505}' => Some((0x36, lines::heavy_triple_dash_horizontal)),
            '\u{2506}' => Some((0x68, lines::light_triple_dash_vertical)),
            '\u{2507}' => Some((0x37, lines::heavy_triple_dash_vertical)),
            '\u{2508}' => Some((0x64, lines::light_quadruple_dash_horizontal)),
            '\u{2509}' => Some((0x33, lines::heavy_quadruple_dash_horizontal)),
            '\u{250A}' => Some((0x65, lines::light_quadruple_dash_vertical)),
            '\u{250B}' => Some((0x34, lines::heavy_quadruple_dash_vertical)),
            '\u{250C}' => Some((0x5D, lines::light_down_and_right)),
            '\u{250D}' => Some((0x20, lines::down_light_and_right_heavy)),
            '\u{250E}' => Some((0x14, lines::down_heavy_and_right_light)),
            '\u{250F}' => Some((0x2D, lines::heavy_down_and_right)),
            '\u{2510}' => Some((0x5C, lines::light_down_and_left)),
            '\u{2511}' => Some((0x1C, lines::down_light_and_left_heavy)),
            '\u{2512}' => Some((0x10, lines::down_heavy_and_left_light)),
            '\u{2513}' => Some((0x2C, lines::heavy_down_and_left)),
            '\u{2514}' => Some((0x6D, lines::light_up_and_right)),
            '\u{2515}' => Some((0x9C, lines::up_light_and_right_heavy)),
            '\u{2516}' => Some((0x90, lines::up_heavy_and_right_light)),
            '\u{2517}' => Some((0x3C, lines::heavy_up_and_right)),
            '\u{2518}' => Some((0x6C, lines::light_up_and_left)),
            '\u{2519}' => Some((0x98, lines::up_light_and_left_heavy)),
            '\u{251A}' => Some((0x8C, lines::up_heavy_and_left_light)),
            '\u{251B}' => Some((0x39, lines::heavy_up_and_left)),
            '\u{251C}' => Some((0x71, lines::light_vertical_and_right)),
            '\u{251D}' => Some((0xAE, lines::vertical_light_and_right_heavy)),
            '\u{251E}' => Some((0x8E, lines::up_heavy_and_right_down_light)),
            '\u{251F}' => Some((0x16, lines::down_heavy_and_right_up_light)),
            '\u{2520}' => Some((0xA8, lines::vertical_heavy_and_right_light)),
            '\u{2521}' => Some((0x22, lines::down_light_and_right_up_heavy)),
            '\u{2522}' => Some((0x9A, lines::up_light_and_right_down_heavy)),
            '\u{2523}' => Some((0x40, lines::heavy_vertical_and_right)),
            '\u{2524}' => Some((0x70, lines::light_vertical_and_left)),
            '\u{2525}' => Some((0xAC, lines::vertical_light_and_left_heavy)),
            '\u{2526}' => Some((0x8A, lines::up_heavy_and_left_down_light)),
            '\u{2527}' => Some((0x12, lines::down_heavy_and_left_up_light)),
            '\u{2528}' => Some((0xA6, lines::vertical_heavy_and_left_light)),
            '\u{2529}' => Some((0x1E, lines::down_light_and_left_up_heavy)),
            '\u{252A}' => Some((0x96, lines::up_light_and_left_down_heavy)),
            '\u{252B}' => Some((0x3F, lines::heavy_vertical_and_left)),
            '\u{252C}' => Some((0x5B, lines::light_down_and_horizontal)),
            '\u{252D}' => Some((0x44, lines::left_heavy_and_right_down_light)),
            '\u{252E}' => Some((0x75, lines::right_heavy_and_left_down_light)),
            '\u{252F}' => Some((0x1A, lines::down_light_and_horizontal_heavy)),
            '\u{2530}' => Some((0x0E, lines::down_heavy_and_horizontal_light)),
            '\u{2531}' => Some((0x7B, lines::right_light_and_left_down_heavy)),
            '\u{2532}' => Some((0x4A, lines::left_light_and_right_down_heavy)),
            '\u{2533}' => Some((0x2B, lines::heavy_down_and_horizontal)),
            '\u{2534}' => Some((0x6B, lines::light_up_and_horizontal)),
            '\u{2535}' => Some((0x46, lines::left_heavy_and_right_up_light)),
            '\u{2536}' => Some((0x77, lines::right_heavy_and_left_up_light)),
            '\u{2537}' => Some((0x94, lines::up_light_and_horizontal_heavy)),
            '\u{2538}' => Some((0x88, lines::up_heavy_and_horizontal_light)),
            '\u{2539}' => Some((0x7D, lines::right_light_and_left_up_heavy)),
            '\u{253A}' => Some((0x4C, lines::left_light_and_right_up_heavy)),
            '\u{253B}' => Some((0x38, lines::heavy_up_and_horizontal)),
            '\u{253C}' => Some((0x6F, lines::light_vertical_and_horizontal)),
            '\u{253D}' => Some((0x48, lines::left_heavy_and_right_vertical_light)),
            '\u{253E}' => Some((0x79, lines::right_heavy_and_left_vertical_light)),
            '\u{253F}' => Some((0xAA, lines::vertical_light_and_horizontal_heavy)),
            '\u{2540}' => Some((0x86, lines::up_heavy_and_down_horizontal_light)),
            '\u{2541}' => Some((0x18, lines::down_heavy_and_up_horizontal_light)),
            '\u{2542}' => Some((0xA4, lines::vertical_heavy_and_horizontal_light)),
            '\u{2543}' => Some((0x50, lines::left_up_heavy_and_right_down_light)),
            '\u{2544}' => Some((0x81, lines::right_up_heavy_and_left_down_light)),
            '\u{2545}' => Some((0x42, lines::left_down_heavy_and_right_up_light)),
            '\u{2546}' => Some((0x73, lines::right_down_heavy_and_left_up_light)),
            '\u{2547}' => Some((0x24, lines::down_light_and_up_horizontal_heavy)),
            '\u{2548}' => Some((0x92, lines::up_light_and_down_horizontal_heavy)),
            '\u{2549}' => Some((0x7F, lines::right_light_and_left_vertical_heavy)),
            '\u{254A}' => Some((0x4E, lines::left_light_and_right_vertical_heavy)),
            '\u{254B}' => Some((0x3E, lines::heavy_vertical_and_horizontal)),
            '\u{254C}' => Some((0x59, lines::light_double_dash_horizontal)),
            '\u{254D}' => Some((0x29, lines::heavy_double_dash_horizontal)),
            '\u{254E}' => Some((0x5A, lines::light_double_dash_vertical)),
            '\u{254F}' => Some((0x2A, lines::heavy_double_dash_vertical)),
            '\u{2550}' => Some((0x03, lines::double_horizontal)),
            '\u{2551}' => Some((0x0A, lines::double_vertical)),
            '\u{2552}' => Some((0x28, lines::down_single_and_right_double)),
            '\u{2553}' => Some((0x0D, lines::down_double_and_right_single)),
            '\u{2554}' => Some((0x02, lines::double_down_and_right)),
            '\u{2555}' => Some((0x27, lines::down_single_and_left_double)),
            '\u{2556}' => Some((0x0C, lines::down_double_and_left_single)),
            '\u{2557}' => Some((0x01, lines::double_down_and_left)),
            '\u{2558}' => Some((0xA0, lines::up_single_and_right_double)),
            '\u{2559}' => Some((0x85, lines::up_double_and_right_single)),
            '\u{255A}' => Some((0x06, lines::double_up_and_right)),
            '\u{255B}' => Some((0x9F, lines::up_single_and_left_double)),
            '\u{255C}' => Some((0x84, lines::up_double_and_left_single)),
            '\u{255D}' => Some((0x05, lines::double_up_and_left)),
            '\u{255E}' => Some((0xB2, lines::vertical_single_and_right_double)),
            '\u{255F}' => Some((0xA3, lines::vertical_double_and_right_single)),
            '\u{2560}' => Some((0x09, lines::double_vertical_and_right)),
            '\u{2561}' => Some((0xB1, lines::vertical_single_and_left_double)),
            '\u{2562}' => Some((0xA2, lines::vertical_double_and_left_single)),
            '\u{2563}' => Some((0x08, lines::double_vertical_and_left)),
            '\u{2564}' => Some((0x26, lines::down_single_and_horizontal_double)),
            '\u{2565}' => Some((0x0B, lines::down_double_and_horizontal_single)),
            '\u{2566}' => Some((0x00, lines::double_down_and_horizontal)),
            '\u{2567}' => Some((0x9E, lines::up_single_and_horizontal_double)),
            '\u{2568}' => Some((0x83, lines::up_double_and_horizontal_single)),
            '\u{2569}' => Some((0x04, lines::double_up_and_horizontal)),
            '\u{256A}' => Some((0xB0, lines::vertical_single_and_horizontal_double)),
            '\u{256B}' => Some((0xA1, lines::vertical_double_and_horizontal_single)),
            '\u{256C}' => Some((0x07, lines::double_vertical_and_horizontal)),
            '\u{256D}' => Some((0x53, lines::light_arc_down_and_right)),
            '\u{256E}' => Some((0x52, lines::light_arc_down_and_left)),
            '\u{256F}' => Some((0x54, lines::light_arc_up_and_left)),
            '\u{2570}' => Some((0x55, lines::light_arc_up_and_right)),
            '\u{2571}' => Some((0x58, cross::light_diagonal_upper_right_to_lower_left)),
            '\u{2572}' => Some((0x57, cross::light_diagonal_upper_left_to_lower_right)),
            '\u{2573}' => Some((0x56, cross::light_diagonal_cross)),
            '\u{2574}' => Some((0x63, lines::light_left)),
            '\u{2575}' => Some((0x6E, lines::light_up)),
            '\u{2576}' => Some((0x66, lines::light_right)),
            '\u{2577}' => Some((0x5E, lines::light_down)),
            '\u{2578}' => Some((0x32, lines::heavy_left)),
            '\u{2579}' => Some((0x3D, lines::heavy_up)),
            '\u{257A}' => Some((0x35, lines::heavy_right)),
            '\u{257B}' => Some((0x2E, lines::heavy_down)),
            '\u{257C}' => Some((0x61, lines::light_left_and_heavy_right)),
            '\u{257D}' => Some((0x69, lines::light_up_and_heavy_down)),
            '\u{257E}' => Some((0x30, lines::heavy_left_and_light_right)),
            '\u{257F}' => Some((0x3A, lines::heavy_up_and_light_down)),
            '\u{2580}'..='\u{D7FF}' | '\u{E000}'..='\u{1FBAE}' => None,
            '\u{1FBAF}' => Some((0x60, lines::light_horizontal_with_vertical_stroke)),
            '\u{1FBB0}'.. => None,
        };

        result.map(|(id_offset, render)| Self {
            id: base_id::BOX_DRAWING + id_offset,
            key,
            render,
        })
    }
}
