use swash::zeno::{Mask, PathData, Point, Stroke, Vector};

use crate::mplus::bitmap::units::Grid;

use super::{Chain, ChainList, GlyphMetrics, Image, ImageCluster};

type Points = [[Point; 3]; 3];

fn render_image(data: impl PathData, stroke: &Stroke) -> Image {
    Mask::new(data).style(stroke).render()
}

fn y_crop_image((mut data, mut placement): Image, top: i32, bottom: i32) -> Image {
    let diff_top = top.saturating_sub(placement.top).max(0);
    let diff_top = diff_top.min(placement.height as i32);
    placement.top = placement.top.saturating_add(diff_top);
    placement.height = placement.height.saturating_sub(diff_top as u32);

    let mut data = data.split_off(placement.width as usize * diff_top as usize);
    let height = bottom.saturating_sub(placement.top).max(0);
    let diff_height = placement.height.saturating_sub(height as u32);
    placement.height = placement.height.saturating_sub(diff_height);

    data.truncate(placement.width as usize * placement.height as usize);

    (data, placement)
}

fn diagonal_upper_right_to_lower_left(points: &Points, offset: Vector) -> Chain {
    Chain::from([points[0][2] + offset, points[2][0] + offset])
}

fn diagonal_upper_left_to_lower_right(points: &Points, offset: Vector) -> Chain {
    Chain::from([points[0][0] + offset, points[2][2] + offset])
}

fn diagonal_cross(points: &Points, offset: Vector) -> ChainList {
    ChainList::from([
        diagonal_upper_right_to_lower_left(points, offset),
        diagonal_upper_left_to_lower_right(points, offset),
    ])
}

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, $fn_call_path:path,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let (Grid(points), stroke) = &glyph_metrics.cross;
                let path = $fn_call_path(points, offset);
                let top = points[0][1].y as i32;
                let bottom = points[2][1].y as i32;
                let image = render_image(&path, stroke);
                let image = y_crop_image(image, top, bottom);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    light_diagonal_upper_right_to_lower_left, diagonal_upper_right_to_lower_left,
    light_diagonal_upper_left_to_lower_right, diagonal_upper_left_to_lower_right,
    light_diagonal_cross, diagonal_cross,
}
