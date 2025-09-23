pub mod parts;

use swash::zeno::{Fill, Vector};

use crate::mplus::bitmap::units::Grid;

use super::render_image;
use super::{Block, GlyphMetrics, ImageCluster, MapIndex, Points};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.block;
                let block = parts::$fn_ident(points, offset);
                let image = render_image(&block, Fill::NonZero);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    upper_half_block,
    lower_half_block,

    upper_one_quarter_block,
    lower_one_quarter_block,
    upper_three_quarters_block,
    lower_three_quarters_block,

    upper_one_eighth_block,
    lower_one_eighth_block,
    upper_three_eighths_block,
    lower_three_eighths_block,
    upper_five_eighths_block,
    lower_five_eighths_block,
    upper_seven_eighths_block,
    lower_seven_eighths_block,

    horizontal_one_eighth_block_2,
    horizontal_one_eighth_block_3,
    horizontal_one_eighth_block_4,
    horizontal_one_eighth_block_5,
    horizontal_one_eighth_block_6,
    horizontal_one_eighth_block_7,
}
