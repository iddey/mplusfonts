use swash::zeno::Vector;

use super::{Block, MapIndex, Points};

macro_rules! def_left_and_right {
    (
        $(
            $fn_ident:ident, $delta_x:literal $(/ $max_x:literal)? $(+ $x0_index:expr)?,
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, offset: Vector) -> Block {
                const MAX_X: usize = 1 $(* $max_x)?;

                let x0_index = if $delta_x < 0 { MAX_X } else { 0 $(+ $x0_index)? % MAX_X };
                let x1_index = x0_index.saturating_add_signed($delta_x).clamp(0, MAX_X);

                let y1_index = Points::map_index::<1>(1);
                let x0_index = Points::map_index::<MAX_X>(x0_index);
                let x1_index = Points::map_index::<MAX_X>(x1_index);

                Block(points[0][x0_index] + offset, points[y1_index][x1_index] + offset)
            }
        )*
    }
}

def_left_and_right! {
    left_half_block, 1 / 2,
    right_half_block, 1 / 2 + 1,

    left_one_quarter_block, 1 / 4,
    right_one_quarter_block, 1 / 4 + 3,
    left_three_quarters_block, 3 / 4,
    right_three_quarters_block, 3 / 4 + 1,

    left_one_eighth_block, 1 / 8,
    right_one_eighth_block, 1 / 8 + 7,
    left_three_eighths_block, 3 / 8,
    right_three_eighths_block, 3 / 8 + 5,
    left_five_eighths_block, 5 / 8,
    right_five_eighths_block, 5 / 8 + 3,
    left_seven_eighths_block, 7 / 8,
    right_seven_eighths_block, 7 / 8 + 1,

    vertical_one_eighth_block_2, 1 / 8 + 2 - 1,
    vertical_one_eighth_block_3, 1 / 8 + 3 - 1,
    vertical_one_eighth_block_4, 1 / 8 + 4 - 1,
    vertical_one_eighth_block_5, 1 / 8 + 5 - 1,
    vertical_one_eighth_block_6, 1 / 8 + 6 - 1,
    vertical_one_eighth_block_7, 1 / 8 + 7 - 1,

    left_one_third_block, 1 / 3,
    left_two_thirds_block, 2 / 3,
}
