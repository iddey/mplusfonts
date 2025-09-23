use swash::zeno::{Fill, Vector};

use crate::mplus::bitmap::units::Grid;

use super::render_image;
use super::{Block, BlockList, GlyphMetrics, ImageCluster, MapIndex, Points};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, [$($delta_index:expr),*] $(- $start_index:expr)?,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.block;
                let start_index = 0 $(+ $start_index)?;
                let blocks = [$($delta_index - start_index),*].map(|index| {
                    let x0_index = index % 2;
                    let x1_index = x0_index + 1;
                    let y0_index = index / 2;
                    let y1_index = y0_index + 1;

                    let x0_index = Points::map_index::<2>(x0_index);
                    let x1_index = Points::map_index::<2>(x1_index);
                    let y0_index = Points::map_index::<3>(y0_index);
                    let y1_index = Points::map_index::<3>(y1_index);

                    Block(points[y0_index][x0_index] + offset, points[y1_index][x1_index] + offset)
                });

                let blocks = BlockList::from(blocks);
                let image = render_image(&blocks, Fill::NonZero);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    block_sextant_1, [1] - 1,
    block_sextant_2, [2] - 1,
    block_sextant_3, [3] - 1,
    block_sextant_4, [4] - 1,
    block_sextant_5, [5] - 1,
    block_sextant_6, [6] - 1,

    block_sextant_12, [1, 2] - 1,
    block_sextant_13, [1, 3] - 1,
    block_sextant_14, [1, 4] - 1,
    block_sextant_15, [1, 5] - 1,
    block_sextant_16, [1, 6] - 1,

    block_sextant_23, [2, 3] - 1,
    block_sextant_24, [2, 4] - 1,
    block_sextant_25, [2, 5] - 1,
    block_sextant_26, [2, 6] - 1,
    block_sextant_34, [3, 4] - 1,
    block_sextant_35, [3, 5] - 1,
    block_sextant_36, [3, 6] - 1,
    block_sextant_45, [4, 5] - 1,
    block_sextant_46, [4, 6] - 1,
    block_sextant_56, [5, 6] - 1,

    block_sextant_123, [1, 2, 3] - 1,
    block_sextant_124, [1, 2, 4] - 1,
    block_sextant_125, [1, 2, 5] - 1,
    block_sextant_126, [1, 2, 6] - 1,
    block_sextant_134, [1, 3, 4] - 1,
    block_sextant_136, [1, 3, 6] - 1,
    block_sextant_145, [1, 4, 5] - 1,
    block_sextant_146, [1, 4, 6] - 1,
    block_sextant_156, [1, 5, 6] - 1,

    block_sextant_234, [2, 3, 4] - 1,
    block_sextant_235, [2, 3, 5] - 1,
    block_sextant_236, [2, 3, 6] - 1,
    block_sextant_245, [2, 4, 5] - 1,
    block_sextant_256, [2, 5, 6] - 1,
    block_sextant_345, [3, 4, 5] - 1,
    block_sextant_346, [3, 4, 6] - 1,
    block_sextant_356, [3, 5, 6] - 1,
    block_sextant_456, [4, 5, 6] - 1,

    block_sextant_1234, [1, 2, 3, 4] - 1,
    block_sextant_1235, [1, 2, 3, 5] - 1,
    block_sextant_1236, [1, 2, 3, 6] - 1,
    block_sextant_1245, [1, 2, 4, 5] - 1,
    block_sextant_1246, [1, 2, 4, 6] - 1,
    block_sextant_1256, [1, 2, 5, 6] - 1,
    block_sextant_1345, [1, 3, 4, 5] - 1,
    block_sextant_1346, [1, 3, 4, 6] - 1,
    block_sextant_1356, [1, 3, 5, 6] - 1,
    block_sextant_1456, [1, 4, 5, 6] - 1,

    block_sextant_2345, [2, 3, 4, 5] - 1,
    block_sextant_2346, [2, 3, 4, 6] - 1,
    block_sextant_2356, [2, 3, 5, 6] - 1,
    block_sextant_2456, [2, 4, 5, 6] - 1,
    block_sextant_3456, [3, 4, 5, 6] - 1,

    block_sextant_12345, [1, 2, 3, 4, 5] - 1,
    block_sextant_12346, [1, 2, 3, 4, 6] - 1,
    block_sextant_12356, [1, 2, 3, 5, 6] - 1,
    block_sextant_12456, [1, 2, 4, 5, 6] - 1,
    block_sextant_13456, [1, 3, 4, 5, 6] - 1,
    block_sextant_23456, [2, 3, 4, 5, 6] - 1,
}
