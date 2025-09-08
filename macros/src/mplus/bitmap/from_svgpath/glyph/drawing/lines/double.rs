pub mod parts;

use swash::zeno::Vector;

use crate::mplus::bitmap::units::Grid;

use super::{Chain, ChainList, GlyphMetrics, ImageCluster, Matrix, Points};
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
                let (Grid(points), stroke) = &glyph_metrics.light;
                let chains = ChainList::from(parts::$fn_ident(points, offset, stroke));
                let image = render_image(&chains, stroke);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    double_horizontal,
    double_vertical,

    double_down_and_right,
    double_down_and_left,

    double_up_and_right,
    double_up_and_left,

    double_vertical_and_right,
    double_vertical_and_left,

    double_down_and_horizontal,
    double_up_and_horizontal,

    double_vertical_and_horizontal,
}
