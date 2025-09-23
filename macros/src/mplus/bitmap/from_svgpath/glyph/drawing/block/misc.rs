use swash::zeno::{Fill, Vector};

use crate::mplus::bitmap::units::Grid;

use super::render_image;
use super::{Block, BlockList, GlyphMetrics, ImageCluster, MapIndex, Points};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident,
            $max_y:literal $(* $max_x:literal)?,
            [$($delta_index:expr),*] $(- $start_index:expr)?,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                const MAX_X: usize = 1 $(* $max_x)?;
                const MAX_Y: usize = $max_y;

                let Grid(points) = &glyph_metrics.block;
                let start_index = 0 $(+ $start_index)?;
                let blocks = [$($delta_index - start_index),*].map(|index| {
                    let x0_index = 0 $(+ index % $max_x)?;
                    let x1_index = x0_index + 1;
                    let y0_index = index $(/ $max_x)?;
                    let y1_index = y0_index + 1;

                    let x0_index = Points::map_index::<MAX_X>(x0_index);
                    let x1_index = Points::map_index::<MAX_X>(x1_index);
                    let y0_index = Points::map_index::<MAX_Y>(y0_index);
                    let y1_index = Points::map_index::<MAX_Y>(y1_index);

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
    quadrant_upper_left, 2 * 2, [1] - 1,
    quadrant_lower_left, 2 * 2, [3] - 1,
    quadrant_upper_left_and_lower_right, 2 * 2, [1, 4] - 1,

    quadrant_upper_right, 2 * 2, [2] - 1,
    quadrant_lower_right, 2 * 2, [4] - 1,
    quadrant_upper_right_and_lower_left, 2 * 2, [2, 3] - 1,

    quadrant_upper_left_and_upper_right_and_lower_left, 2 * 2, [1, 2, 3] - 1,
    quadrant_upper_left_and_upper_right_and_lower_right, 2 * 2, [1, 2, 4] - 1,
    quadrant_upper_left_and_lower_left_and_lower_right, 2 * 2, [1, 3, 4] - 1,
    quadrant_upper_right_and_lower_left_and_lower_right, 2 * 2, [2, 3, 4] - 1,

    upper_left_one_sixteenth_block, 4 * 4, [1] - 1,
    upper_centre_left_one_sixteenth_block, 4 * 4, [2] - 1,
    upper_centre_right_one_sixteenth_block, 4 * 4, [3] - 1,
    upper_right_one_sixteenth_block, 4 * 4, [4] - 1,

    upper_middle_left_one_sixteenth_block, 4 * 4, [5] - 1,
    upper_middle_centre_left_one_sixteenth_block, 4 * 4, [6] - 1,
    upper_middle_centre_right_one_sixteenth_block, 4 * 4, [7] - 1,
    upper_middle_right_one_sixteenth_block, 4 * 4, [8] - 1,

    lower_middle_left_one_sixteenth_block, 4 * 4, [9] - 1,
    lower_middle_centre_left_one_sixteenth_block, 4 * 4, [10] - 1,
    lower_middle_centre_right_one_sixteenth_block, 4 * 4, [11] - 1,
    lower_middle_right_one_sixteenth_block, 4 * 4, [12] - 1,

    lower_left_one_sixteenth_block, 4 * 4, [13] - 1,
    lower_centre_left_one_sixteenth_block, 4 * 4, [14] - 1,
    lower_centre_right_one_sixteenth_block, 4 * 4, [15] - 1,
    lower_right_one_sixteenth_block, 4 * 4, [16] - 1,

    right_half_lower_one_quarter_block, 4 * 2, [8] - 1,
    right_three_quarters_lower_one_quarter_block, 4 * 4, [6, 7, 8] - 1,
    left_three_quarters_lower_one_quarter_block, 4 * 4, [5, 6, 7] - 1,
    left_half_lower_one_quarter_block, 4 * 2, [7] - 1,

    lower_half_left_one_quarter_block, 2 * 4, [5] - 1,
    lower_three_quarters_left_one_quarter_block, 4 * 4, [5, 9, 13] - 1,
    upper_three_quarters_left_one_quarter_block, 4 * 4, [1, 5, 9] - 1,
    upper_half_left_one_quarter_block, 2 * 4, [1] - 1,

    left_half_upper_one_quarter_block, 4 * 2, [1] - 1,
    left_three_quarters_upper_one_quarter_block, 4 * 4, [1, 2, 3] - 1,
    right_three_quarters_upper_one_quarter_block, 4 * 4, [2, 3, 4] - 1,
    right_half_upper_one_quarter_block, 4 * 2, [2] - 1,

    lower_half_right_one_quarter_block, 2 * 4, [8] - 1,
    lower_three_quarters_right_one_quarter_block, 4 * 4, [8, 12, 16] - 1,
    upper_three_quarters_right_one_quarter_block, 4 * 4, [4, 8, 12] - 1,
    upper_half_right_one_quarter_block, 2 * 4, [4] - 1,

    upper_centre_one_quarter_block, 2 * 4, [2, 3] - 1,
    lower_centre_one_quarter_block, 2 * 4, [6, 7] - 1,
    middle_left_one_quarter_block, 4 * 2, [3, 5] - 1,
    middle_right_one_quarter_block, 4 * 2, [4, 6] - 1,

    checker_board_fill, 4 * 4, [1, 3, 6, 8, 9, 11, 14, 16] - 1,
    inverse_checker_board_fill, 4 * 4, [2, 4, 5, 7, 10, 12, 13, 15] - 1,

    heavy_horizontal_fill, 4, [2, 4] - 1,
}
