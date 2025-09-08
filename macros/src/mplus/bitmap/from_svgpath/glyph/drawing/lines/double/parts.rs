use swash::zeno::{Stroke, Vector};

use super::super::single::parts::*;
use super::{Chain, Matrix, Points};

fn warp_points(points: &Points, middle_offset: Vector) -> Points {
    let middle_column_offset = Vector::new(middle_offset.x, 0.0);
    let middle_row_offset = Vector::new(0.0, middle_offset.y);
    let middle_row = [
        points[1][0] + middle_row_offset,
        points[1][1] + middle_row_offset + middle_column_offset,
        points[1][2] + middle_row_offset,
    ];

    let [top_row, bottom_row] = [0, 2].map(|y_index| {
        [
            points[y_index][0],
            points[y_index][1] + middle_column_offset,
            points[y_index][2],
        ]
    });

    [top_row, middle_row, bottom_row]
}

fn x_and_y_offset(stroke: &Stroke, x_signum: f32, y_signum: f32) -> Vector {
    Vector::new(x_signum * stroke.width, y_signum * stroke.width)
}

fn x_offset(stroke: &Stroke, x_signum: f32) -> Vector {
    Vector::new(x_signum * stroke.width, 0.0)
}

fn y_offset(stroke: &Stroke, y_signum: f32) -> Vector {
    Vector::new(0.0, y_signum * stroke.width)
}

macro_rules! def_half_of_double {
    (
        $(
            $fn_ident:ident,
            $fn_call_path:path,
            $middle_offset_ident:ident($($signum:expr),*),
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, offset: Matrix, stroke: &Stroke) -> Chain {
                let middle_offset = $middle_offset_ident(stroke, $($signum),*);
                let points = warp_points(points, middle_offset);

                $fn_call_path(&points, offset)
            }
        )*
    }
}

def_half_of_double! {
    bottom_half_of_double_horizontal, horizontal, y_offset(1.0),
    right_half_of_double_vertical, vertical, x_offset(1.0),
    left_half_of_double_vertical, vertical, x_offset(-1.0),
    top_half_of_double_horizontal, horizontal, y_offset(-1.0),

    bottom_half_of_double_left, left, y_offset(1.0),
    right_half_of_double_up, up, x_offset(1.0),
    left_half_of_double_up, up, x_offset(-1.0),
    top_half_of_double_left, left, y_offset(-1.0),

    bottom_half_of_double_right, right, y_offset(1.0),
    right_half_of_double_down, down, x_offset(1.0),
    left_half_of_double_down, down, x_offset(-1.0),
    top_half_of_double_right, right, y_offset(-1.0),

    bottom_right_half_of_double_down_and_right, down_and_right, x_and_y_offset(1.0, 1.0),
    bottom_left_half_of_double_down_and_left, down_and_left, x_and_y_offset(-1.0, 1.0),
    top_right_half_of_double_up_and_right, up_and_right, x_and_y_offset(1.0, -1.0),
    top_left_half_of_double_up_and_left, up_and_left, x_and_y_offset(-1.0, -1.0),

    bottom_right_half_of_double_up_and_left, up_and_left, x_and_y_offset(1.0, 1.0),
    bottom_left_half_of_double_up_and_right, up_and_right, x_and_y_offset(-1.0, 1.0),
    top_right_half_of_double_down_and_left, down_and_left, x_and_y_offset(1.0, -1.0),
    top_left_half_of_double_down_and_right, down_and_right, x_and_y_offset(-1.0, -1.0),
}

macro_rules! def_double {
    (
        $(
            $fn_ident:ident, [$($fn_call_path:path),* $(,)?],
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, offset: Matrix, stroke: &Stroke) -> Vec<Chain> {
                Vec::from([$($fn_call_path(points, offset, stroke)),*])
            }
        )*
    }
}

def_double! {
    double_horizontal, [
        bottom_half_of_double_horizontal,
        top_half_of_double_horizontal
    ],
    double_vertical, [
        right_half_of_double_vertical,
        left_half_of_double_vertical
    ],

    double_down_and_right, [
        bottom_right_half_of_double_down_and_right,
        top_left_half_of_double_down_and_right,
    ],
    double_down_and_left, [
        bottom_left_half_of_double_down_and_left,
        top_right_half_of_double_down_and_left,
    ],

    double_up_and_right, [
        bottom_left_half_of_double_up_and_right,
        top_right_half_of_double_up_and_right,
    ],
    double_up_and_left, [
        bottom_right_half_of_double_up_and_left,
        top_left_half_of_double_up_and_left,
    ],

    double_vertical_and_right, [
        bottom_right_half_of_double_down_and_right,
        top_right_half_of_double_up_and_right,
        left_half_of_double_vertical,
    ],
    double_vertical_and_left, [
        bottom_left_half_of_double_down_and_left,
        top_left_half_of_double_up_and_left,
        right_half_of_double_vertical,
    ],

    double_down_and_horizontal, [
        top_half_of_double_horizontal,
        bottom_right_half_of_double_down_and_right,
        bottom_left_half_of_double_down_and_left,
    ],
    double_up_and_horizontal, [
        bottom_half_of_double_horizontal,
        top_right_half_of_double_up_and_right,
        top_left_half_of_double_up_and_left,
    ],

    double_vertical_and_horizontal, [
        bottom_right_half_of_double_down_and_right,
        bottom_left_half_of_double_down_and_left,
        top_right_half_of_double_up_and_right,
        top_left_half_of_double_up_and_left,
    ],

    double_left, [
        bottom_half_of_double_left,
        top_half_of_double_left
    ],
    double_up, [
        right_half_of_double_up,
        left_half_of_double_up
    ],

    double_right, [
        bottom_half_of_double_right,
        top_half_of_double_right
    ],
    double_down, [
        right_half_of_double_down,
        left_half_of_double_down
    ],
}

macro_rules! def_short_or_long {
    (
        $(
            $fn_ident:ident, $fn_call_path:path, $center_offset_ident:ident($signum:expr),
        )*
    ) => {
        $(
            pub fn $fn_ident(points: &Points, mut offset: Matrix, stroke: &Stroke) -> Chain {
                let center_offset = $center_offset_ident(stroke, $signum);
                offset[1][1] = offset[1][1] + center_offset;

                $fn_call_path(points, offset)
            }
        )*
    }
}

def_short_or_long! {
    short_left, left, x_offset(-1.0),
    long_left, left, x_offset(1.0),
    short_up, up, y_offset(-1.0),
    long_up, up, y_offset(1.0),

    short_right, right, x_offset(1.0),
    long_right, right, x_offset(-1.0),
    short_down, down, y_offset(1.0),
    long_down, down, y_offset(-1.0),
}
