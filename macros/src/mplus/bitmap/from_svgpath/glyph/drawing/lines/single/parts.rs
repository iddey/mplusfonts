use super::{Chain, ChainList, Matrix, Points};

pub fn horizontal(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[1][0] + offset[1][0], points[1][2] + offset[1][0]])
}

pub fn vertical(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[0][1] + offset[0][1], points[2][1] + offset[0][1]])
}

pub fn vertical_stroke(points: &Points, offset: Matrix) -> Chain {
    Chain::from([
        (points[0][1] + points[1][1]) / 2.0 + offset[1][1],
        (points[1][1] + points[2][1]) / 2.0 + offset[1][1],
    ])
}

pub fn left(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[1][1] + offset[1][1], points[1][0] + offset[1][0]])
}

pub fn up(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[0][1] + offset[0][1], points[1][1] + offset[1][1]])
}

pub fn right(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[1][1] + offset[1][1], points[1][2] + offset[1][0]])
}

pub fn down(points: &Points, offset: Matrix) -> Chain {
    Chain::from([points[2][1] + offset[0][1], points[1][1] + offset[1][1]])
}

pub fn down_and_right(points: &Points, offset: Matrix) -> Chain {
    Chain::from([
        points[2][1] + offset[0][1],
        points[1][1] + offset[1][1],
        points[1][2] + offset[1][0],
    ])
}

pub fn down_and_left(points: &Points, offset: Matrix) -> Chain {
    Chain::from([
        points[2][1] + offset[0][1],
        points[1][1] + offset[1][1],
        points[1][0] + offset[1][0],
    ])
}

pub fn up_and_right(points: &Points, offset: Matrix) -> Chain {
    Chain::from([
        points[0][1] + offset[0][1],
        points[1][1] + offset[1][1],
        points[1][2] + offset[1][0],
    ])
}

pub fn up_and_left(points: &Points, offset: Matrix) -> Chain {
    Chain::from([
        points[0][1] + offset[0][1],
        points[1][1] + offset[1][1],
        points[1][0] + offset[1][0],
    ])
}

macro_rules! def_complex_part {
    (
        $(
            $fn_ident:ident, [$($fn_call_path:path),* $(,)?],
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, offset: Matrix) -> ChainList {
                ChainList::from([$($fn_call_path(points, offset)),*])
            }
        )*
    }
}

def_complex_part! {
    vertical_and_right, [right, vertical],
    vertical_and_left, [left, vertical],

    down_and_horizontal, [horizontal, down],
    up_and_horizontal, [horizontal, up],

    vertical_and_horizontal, [horizontal, vertical],

    horizontal_with_vertical_stroke, [horizontal, vertical_stroke],
}
