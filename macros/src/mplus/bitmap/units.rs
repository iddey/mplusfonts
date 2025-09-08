use std::array;

use swash::zeno::Point;

use crate::mplus::font::{Font, FontWidth};

#[derive(Clone, Copy)]
pub enum Halfwidth {
    Floor(f32),
    Ceil,
    Zero,
}

pub struct Grid<const N: usize>(pub [[Point; N]; N]);

impl Halfwidth {
    pub fn from_font(font: &Font, pixels_per_em: f32) -> Self {
        let em_per_halfwidth = match *font {
            Font::MPLUSCode {
                variable: (.., FontWidth(units)),
                ..
            } => f32::from(units).mul_add(0.4 / 100.0, 0.1),
            _ => 0.5,
        };

        match pixels_per_em {
            ..1.25 => Self::Zero,
            ..2.0 => Self::Ceil,
            _ => Self::Floor(pixels_per_em * em_per_halfwidth),
        }
    }
}

impl<const N: usize> Grid<N> {
    pub fn new(width: f32, height: f32, snap: Option<(f32, f32)>) -> Self {
        let target_x = width.floor();
        let target_y = height.floor();
        let x_scale = if width > 0.0 { target_x / width } else { 0.0 };
        let y_scale = if height > 0.0 { target_y / height } else { 0.0 };
        let x_increment = if N < 2 { 0.0 } else { width / (N - 1) as f32 };
        let y_increment = if N < 2 { 0.0 } else { height / (N - 1) as f32 };

        let mut x = 0f32;
        let mut y = 0f32;
        let coords: [(f32, f32); N] = array::from_fn(|_| {
            let coords = if let Some((x_translate, y_translate)) = snap {
                let x = (x.mul_add(x_scale, x_translate)).round() - x_translate;
                let y = (y.mul_add(y_scale, y_translate)).round() - y_translate;

                (x, y)
            } else {
                (x, y)
            };

            x += x_increment;
            y += y_increment;

            coords
        });

        let points = array::from_fn(|y_index| {
            array::from_fn(|x_index| {
                let (x, _) = coords[x_index];
                let (_, y) = coords[y_index];

                Point::new(x, y)
            })
        });

        Self(points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_new_points {
        (
            $(
                $fn_ident:ident, $n:expr, $width:expr, $height:expr, $snap:expr, $expected:expr,
            )*
        ) => {
            $(
                #[test]
                fn $fn_ident() {
                    let Grid(points) = Grid::<$n>::new($width, $height, $snap);
                    assert_eq!(points, $expected);
                }
            )*
        }
    }

    test_new_points! {
        new_with_0_points, 0, 7.5, 15.0, None, [[]; 0],

        new_with_1_point_and_no_snap,
            1, 7.5, 15.0, None, [
                [Point::new(0.0, 0.0)]
            ],

        new_with_4_points_and_no_snap,
            2, 7.5, 15.0, None, [
                [Point::new(0.0, 0.0), Point::new(7.5, 0.0)],
                [Point::new(0.0, 15.0), Point::new(7.5, 15.0)],
            ],

        new_with_9_points_and_no_snap,
            3, 7.5, 15.0, None, [
                [Point::new(0.0, 0.0), Point::new(3.75, 0.0), Point::new(7.5, 0.0)],
                [Point::new(0.0, 7.5), Point::new(3.75, 7.5), Point::new(7.5, 7.5)],
                [Point::new(0.0, 15.0), Point::new(3.75, 15.0), Point::new(7.5, 15.0)],
            ],

        new_with_1_point_and_snap_to_grid,
            1, 7.5, 15.0, Some((0.0, 0.0)), [
                [Point::new(0.0, 0.0)]
            ],

        new_with_4_points_and_snap_to_grid,
            2, 7.5, 15.0, Some((0.0, 0.0)), [
                [Point::new(0.0, 0.0), Point::new(7.0, 0.0)],
                [Point::new(0.0, 15.0), Point::new(7.0, 15.0)],
            ],

        new_with_9_points_and_snap_to_grid,
            3, 7.5, 15.0, Some((0.0, 0.0)), [
                [Point::new(0.0, 0.0), Point::new(4.0, 0.0), Point::new(7.0, 0.0)],
                [Point::new(0.0, 8.0), Point::new(4.0, 8.0), Point::new(7.0, 8.0)],
                [Point::new(0.0, 15.0), Point::new(4.0, 15.0), Point::new(7.0, 15.0)],
            ],

        new_with_1_point_and_snap_in_between,
            1, 7.5, 15.0, Some((0.5, 0.5)), [
                [Point::new(0.5, 0.5)]
            ],

        new_with_4_points_and_snap_in_between,
            2, 7.5, 15.0, Some((0.5, 0.5)), [
                [Point::new(0.5, 0.5), Point::new(7.5, 0.5)],
                [Point::new(0.5, 15.5), Point::new(7.5, 15.5)],
            ],

        new_with_9_points_and_snap_in_between,
            3, 7.5, 15.0, Some((0.5, 0.5)), [
                [Point::new(0.5, 0.5), Point::new(3.5, 0.5), Point::new(7.5, 0.5)],
                [Point::new(0.5, 7.5), Point::new(3.5, 7.5), Point::new(7.5, 7.5)],
                [Point::new(0.5, 15.5), Point::new(3.5, 15.5), Point::new(7.5, 15.5)],
            ],
    }
}
