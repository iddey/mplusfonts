use crate::mplus::bitmap::units::Grid;

use super::parts::*;
use super::render_image;
use super::{GlyphMetrics, Image, Matrix};

macro_rules! def_light_and_heavy {
    (
        $(
            $fn_ident:ident,
            $fn_call_path:path,
            $glyph_metrics_ident:ident,
            $glyph_metrics_field:expr,
        )*
    ) => {
        $(
            pub fn $fn_ident($glyph_metrics_ident: &GlyphMetrics, offset: Matrix) -> Image {
                let (Grid(points), stroke) = $glyph_metrics_field;
                let chains = $fn_call_path(points, offset);

                render_image(&chains, stroke)
            }
        )*
    }
}

def_light_and_heavy! {
    light_vertical_and_right, vertical_and_right, glyph_metrics, &glyph_metrics.light,
    heavy_vertical_and_right, vertical_and_right, glyph_metrics, &glyph_metrics.heavy,
    light_vertical_and_left, vertical_and_left, glyph_metrics, &glyph_metrics.light,
    heavy_vertical_and_left, vertical_and_left, glyph_metrics, &glyph_metrics.heavy,

    light_down_and_horizontal, down_and_horizontal, glyph_metrics, &glyph_metrics.light,
    heavy_down_and_horizontal, down_and_horizontal, glyph_metrics, &glyph_metrics.heavy,
    light_up_and_horizontal, up_and_horizontal, glyph_metrics, &glyph_metrics.light,
    heavy_up_and_horizontal, up_and_horizontal, glyph_metrics, &glyph_metrics.heavy,

    light_vertical_and_horizontal, vertical_and_horizontal, glyph_metrics, &glyph_metrics.light,
    heavy_vertical_and_horizontal, vertical_and_horizontal, glyph_metrics, &glyph_metrics.heavy,

    light_horizontal_with_vertical_stroke,
        horizontal_with_vertical_stroke,
        glyph_metrics,
        &glyph_metrics.light,
}
