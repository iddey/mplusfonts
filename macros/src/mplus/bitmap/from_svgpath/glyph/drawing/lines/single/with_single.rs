pub mod renders;

use swash::zeno::Vector;

use super::super::single::parts;
use super::{GlyphMetrics, Image, ImageCluster, Matrix};
use super::{offset_table, render_image};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let offset = offset_table(offset);

                Vec::from([renders::$fn_ident(glyph_metrics, offset)])
            }
        )*
    }
}

def_unicode_char! {
    light_vertical_and_right,
    heavy_vertical_and_right,
    light_vertical_and_left,
    heavy_vertical_and_left,

    light_down_and_horizontal,
    heavy_down_and_horizontal,
    light_up_and_horizontal,
    heavy_up_and_horizontal,

    light_vertical_and_horizontal,
    heavy_vertical_and_horizontal,

    light_horizontal_with_vertical_stroke,
}
