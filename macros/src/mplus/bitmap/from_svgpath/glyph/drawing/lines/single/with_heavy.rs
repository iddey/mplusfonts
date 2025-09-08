pub mod renders;

use swash::zeno::Vector;

use super::super::single::parts;
use super::{GlyphMetrics, Image, ImageCluster, Matrix};
use super::{offset_table, render_image};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, [$($fn_call_path:path),* $(,)?],
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let offset = offset_table(offset);

                Vec::from([$($fn_call_path(glyph_metrics, offset)),*])
            }
        )*
    }
}

def_unicode_char! {
    down_light_and_right_heavy, [renders::heavy_right, renders::light_down],
    down_heavy_and_right_light, [renders::light_right, renders::heavy_down],
    down_light_and_left_heavy, [renders::heavy_left, renders::light_down],
    down_heavy_and_left_light, [renders::light_left, renders::heavy_down],

    up_light_and_right_heavy, [renders::heavy_right, renders::light_up],
    up_heavy_and_right_light, [renders::light_right, renders::heavy_up],
    up_light_and_left_heavy, [renders::heavy_left, renders::light_up],
    up_heavy_and_left_light, [renders::light_left, renders::heavy_up],

    vertical_light_and_right_heavy, [renders::heavy_right, super::renders::light_vertical],
    up_heavy_and_right_down_light, [super::renders::light_down_and_right, renders::heavy_up],
    down_heavy_and_right_up_light, [super::renders::light_up_and_right, renders::heavy_down],
    vertical_heavy_and_right_light, [renders::light_right, super::renders::heavy_vertical],
    down_light_and_right_up_heavy, [super::renders::heavy_up_and_right, renders::light_down],
    up_light_and_right_down_heavy, [super::renders::heavy_down_and_right, renders::light_up],

    vertical_light_and_left_heavy, [renders::heavy_left, super::renders::light_vertical],
    up_heavy_and_left_down_light, [super::renders::light_down_and_left, renders::heavy_up],
    down_heavy_and_left_up_light, [super::renders::light_up_and_left, renders::heavy_down],
    vertical_heavy_and_left_light, [renders::light_left, super::renders::heavy_vertical],
    down_light_and_left_up_heavy, [super::renders::heavy_up_and_left, renders::light_down],
    up_light_and_left_down_heavy, [super::renders::heavy_down_and_left, renders::light_up],

    left_heavy_and_right_down_light, [super::renders::light_down_and_right, renders::heavy_left],
    right_heavy_and_left_down_light, [super::renders::light_down_and_left, renders::heavy_right],
    down_light_and_horizontal_heavy, [super::renders::heavy_horizontal, renders::light_down],
    down_heavy_and_horizontal_light, [super::renders::light_horizontal, renders::heavy_down],
    right_light_and_left_down_heavy, [super::renders::heavy_down_and_left, renders::light_right],
    left_light_and_right_down_heavy, [super::renders::heavy_down_and_right, renders::light_left],

    left_heavy_and_right_up_light, [super::renders::light_up_and_right, renders::heavy_left],
    right_heavy_and_left_up_light, [super::renders::light_up_and_left, renders::heavy_right],
    up_light_and_horizontal_heavy, [super::renders::heavy_horizontal, renders::light_up],
    up_heavy_and_horizontal_light, [super::renders::light_horizontal, renders::heavy_up],
    right_light_and_left_up_heavy, [super::renders::heavy_up_and_left, renders::light_right],
    left_light_and_right_up_heavy, [super::renders::heavy_up_and_right, renders::light_left],

    left_heavy_and_right_vertical_light,
        [super::with_single::renders::light_vertical_and_right, renders::heavy_left],
    right_heavy_and_left_vertical_light,
        [super::with_single::renders::light_vertical_and_left, renders::heavy_right],
    vertical_light_and_horizontal_heavy,
        [super::renders::heavy_horizontal, super::renders::light_vertical],
    up_heavy_and_down_horizontal_light,
        [super::with_single::renders::light_down_and_horizontal, renders::heavy_up],
    down_heavy_and_up_horizontal_light,
        [super::with_single::renders::light_up_and_horizontal, renders::heavy_down],
    vertical_heavy_and_horizontal_light,
        [super::renders::light_horizontal, super::renders::heavy_vertical],

    left_up_heavy_and_right_down_light,
        [super::renders::light_down_and_right, super::renders::heavy_up_and_left],
    right_up_heavy_and_left_down_light,
        [super::renders::light_down_and_left, super::renders::heavy_up_and_right],
    left_down_heavy_and_right_up_light,
        [super::renders::light_up_and_right, super::renders::heavy_down_and_left],
    right_down_heavy_and_left_up_light,
        [super::renders::light_up_and_left, super::renders::heavy_down_and_right],

    down_light_and_up_horizontal_heavy,
        [super::with_single::renders::heavy_up_and_horizontal, renders::light_down],
    up_light_and_down_horizontal_heavy,
        [super::with_single::renders::heavy_down_and_horizontal, renders::light_up],
    right_light_and_left_vertical_heavy,
        [super::with_single::renders::heavy_vertical_and_left, renders::light_right],
    left_light_and_right_vertical_heavy,
        [super::with_single::renders::heavy_vertical_and_right, renders::light_left],

    light_left_and_heavy_right, [renders::heavy_right, renders::light_left],
    light_up_and_heavy_down, [renders::heavy_down, renders::light_up],
    heavy_left_and_light_right, [renders::light_right, renders::heavy_left],
    heavy_up_and_light_down, [renders::light_down, renders::heavy_up],
}
