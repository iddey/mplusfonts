use swash::zeno::{Stroke, Vector};

use crate::mplus::bitmap::units::Grid;

use super::parts::*;
use super::render_image;
use super::{GlyphMetrics, Image, Matrix};

fn center_offset(tuple: &(Grid<3>, Stroke), other: &(Grid<3>, Stroke), signum: f32) -> Vector {
    let (Grid(points), stroke) = tuple;
    let (Grid(other_points), other_stroke) = other;
    let diff_center_points = other_points[1][1] - points[1][1];
    let diff_square_cap_heights = other_stroke.width / 2.0 - stroke.width / 2.0;

    Vector::from(signum * diff_square_cap_heights) + diff_center_points
}

fn x_offset(tuple: &(Grid<3>, Stroke), other: &(Grid<3>, Stroke), signum: f32) -> Vector {
    let x_offset = center_offset(tuple, other, signum).x;

    Vector::new(x_offset, 0.0)
}

fn y_offset(tuple: &(Grid<3>, Stroke), other: &(Grid<3>, Stroke), signum: f32) -> Vector {
    let y_offset = center_offset(tuple, other, signum).y;

    Vector::new(0.0, y_offset)
}

macro_rules! def_light_or_heavy {
    (
        $(
            $fn_ident:ident,
            $fn_call_path:path,
            $glyph_metrics_ident:ident,
            $glyph_metrics_field:expr,
            $glyph_metrics_other:expr,
            $center_offset_ident:ident($signum:expr),
        )*
    ) => {
        $(
            pub fn $fn_ident($glyph_metrics_ident: &GlyphMetrics, mut offset: Matrix) -> Image {
                let tuple = $glyph_metrics_field;
                let other = $glyph_metrics_other;
                offset[1][1] = offset[1][1] + $center_offset_ident(tuple, other, $signum);

                let (Grid(points), stroke) = tuple;
                let chain = $fn_call_path(points, offset);

                render_image(&chain, stroke)
            }
        )*
    }
}

def_light_or_heavy! {
    light_left, left, glyph_metrics, &glyph_metrics.light, &glyph_metrics.heavy, x_offset(1.0),
    heavy_left, left, glyph_metrics, &glyph_metrics.heavy, &glyph_metrics.light, x_offset(1.0),
    light_up, up, glyph_metrics, &glyph_metrics.light, &glyph_metrics.heavy, y_offset(1.0),
    heavy_up, up, glyph_metrics, &glyph_metrics.heavy, &glyph_metrics.light, y_offset(1.0),

    light_right, right, glyph_metrics, &glyph_metrics.light, &glyph_metrics.heavy, x_offset(-1.0),
    heavy_right, right, glyph_metrics, &glyph_metrics.heavy, &glyph_metrics.light, x_offset(-1.0),
    light_down, down, glyph_metrics, &glyph_metrics.light, &glyph_metrics.heavy, y_offset(-1.0),
    heavy_down, down, glyph_metrics, &glyph_metrics.heavy, &glyph_metrics.light, y_offset(-1.0),
}
