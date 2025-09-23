use swash::zeno::Vector;

use super::{Block, MapIndex, Points};

macro_rules! def_upper_and_lower {
    (
        $(
            $fn_ident:ident, $delta_y:literal $(/ $max_y:literal)? $(+ $y0_index:expr)?,
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, offset: Vector) -> Block {
                const MAX_Y: usize = 1 $(* $max_y)?;

                let y0_index = if $delta_y < 0 { MAX_Y } else { 0 $(+ $y0_index)? % MAX_Y };
                let y1_index = y0_index.saturating_add_signed($delta_y).clamp(0, MAX_Y);

                let x1_index = Points::map_index::<1>(1);
                let y0_index = Points::map_index::<MAX_Y>(y0_index);
                let y1_index = Points::map_index::<MAX_Y>(y1_index);

                Block(points[y0_index][0] + offset, points[y1_index][x1_index] + offset)
            }
        )*
    }
}

def_upper_and_lower! {
    upper_half_block, 1 / 2,
    lower_half_block, 1 / 2 + 1,

    upper_one_quarter_block, 1 / 4,
    lower_one_quarter_block, 1 / 4 + 3,
    upper_three_quarters_block, 3 / 4,
    lower_three_quarters_block, 3 / 4 + 1,

    upper_one_eighth_block, 1 / 8,
    lower_one_eighth_block, 1 / 8 + 7,
    upper_three_eighths_block, 3 / 8,
    lower_three_eighths_block, 3 / 8 + 5,
    upper_five_eighths_block, 5 / 8,
    lower_five_eighths_block, 5 / 8 + 3,
    upper_seven_eighths_block, 7 / 8,
    lower_seven_eighths_block, 7 / 8 + 1,

    horizontal_one_eighth_block_2, 1 / 8 + 2 - 1,
    horizontal_one_eighth_block_3, 1 / 8 + 3 - 1,
    horizontal_one_eighth_block_4, 1 / 8 + 4 - 1,
    horizontal_one_eighth_block_5, 1 / 8 + 5 - 1,
    horizontal_one_eighth_block_6, 1 / 8 + 6 - 1,
    horizontal_one_eighth_block_7, 1 / 8 + 7 - 1,
}
