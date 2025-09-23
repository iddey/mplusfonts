use swash::zeno::{Fill, Vector};

use crate::mplus::bitmap::units::Grid;

use super::render_image;
use super::tall::parts::*;
use super::wide::parts::*;
use super::{BlockList, GlyphMetrics, ImageCluster};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, [$($fn_call_path:path),* $(,)?],
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.block;
                let blocks = BlockList::from([$($fn_call_path(points, offset)),*]);
                let image = render_image(&blocks, Fill::NonZero);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    left_and_lower_one_eighth_block, [lower_one_eighth_block, left_one_eighth_block],
    left_and_upper_one_eighth_block, [upper_one_eighth_block, left_one_eighth_block],

    right_and_upper_one_eighth_block, [upper_one_eighth_block, right_one_eighth_block],
    right_and_lower_one_eighth_block, [lower_one_eighth_block, right_one_eighth_block],

    upper_and_lower_one_eighth_block, [lower_one_eighth_block, upper_one_eighth_block],

    horizontal_one_eighth_block_1358, [
        lower_one_eighth_block,
        horizontal_one_eighth_block_5,
        horizontal_one_eighth_block_3,
        upper_one_eighth_block
    ],
}
