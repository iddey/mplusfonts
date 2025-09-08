pub mod parts;
pub mod renders;
pub mod with_arc;
pub mod with_double;
pub mod with_heavy;
pub mod with_single;

use swash::zeno::Vector;

use super::{Chain, ChainList, GlyphMetrics, Image, ImageCluster, Matrix, Points};
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
    light_horizontal,
    heavy_horizontal,
    light_vertical,
    heavy_vertical,

    light_double_dash_horizontal,
    heavy_double_dash_horizontal,
    light_double_dash_vertical,
    heavy_double_dash_vertical,

    light_triple_dash_horizontal,
    heavy_triple_dash_horizontal,
    light_triple_dash_vertical,
    heavy_triple_dash_vertical,

    light_quadruple_dash_horizontal,
    heavy_quadruple_dash_horizontal,
    light_quadruple_dash_vertical,
    heavy_quadruple_dash_vertical,

    light_left,
    heavy_left,
    light_up,
    heavy_up,

    light_right,
    heavy_right,
    light_down,
    heavy_down,

    light_down_and_right,
    heavy_down_and_right,
    light_down_and_left,
    heavy_down_and_left,

    light_up_and_right,
    heavy_up_and_right,
    light_up_and_left,
    heavy_up_and_left,
}
