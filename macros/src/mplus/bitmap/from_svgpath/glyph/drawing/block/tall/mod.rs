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
    left_half_block,
    right_half_block,

    left_one_quarter_block,
    right_one_quarter_block,
    left_three_quarters_block,
    right_three_quarters_block,

    left_one_eighth_block,
    right_one_eighth_block,
    left_three_eighths_block,
    right_three_eighths_block,
    left_five_eighths_block,
    right_five_eighths_block,
    left_seven_eighths_block,
    right_seven_eighths_block,

    vertical_one_eighth_block_2,
    vertical_one_eighth_block_3,
    vertical_one_eighth_block_4,
    vertical_one_eighth_block_5,
    vertical_one_eighth_block_6,
    vertical_one_eighth_block_7,

    left_one_third_block,
    left_two_thirds_block,
}
