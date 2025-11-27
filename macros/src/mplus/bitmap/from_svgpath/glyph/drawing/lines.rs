mod double;
mod single;

use swash::zeno::{Mask, PathData, Point, Stroke, Vector};

use super::{Chain, ChainList, GlyphMetrics, Image, ImageCluster};

pub use double::*;
pub use single::with_arc::*;
pub use single::with_double::*;
pub use single::with_heavy::*;
pub use single::with_single::*;
pub use single::*;

type Points = [[Point; 3]; 3];
type Matrix = [[Vector; 2]; 2];

fn offset_table(offset: Vector) -> Matrix {
    let mut offset_table = [[offset, offset], [offset, offset]];
    offset_table[0][1].x = offset_table[0][1].x.round();
    offset_table[1][0].y = offset_table[1][0].y.round();
    offset_table[1][1].x = offset_table[0][1].x;
    offset_table[1][1].y = offset_table[1][0].y;

    offset_table
}

fn render_image(data: impl PathData, stroke: &Stroke) -> Image {
    Mask::new(data).style(stroke).render()
}

fn trim_image((mut data, mut placement): Image) -> Image {
    let data_top = data.iter();
    let diff_top = data_top
        .take_while(|value| **value == 0)
        .count()
        .checked_div(placement.width as usize)
        .unwrap_or_default();

    let diff_top = diff_top.try_into().unwrap_or(u32::MAX);
    let diff_top = diff_top.min(placement.height);
    placement.top = placement.top.saturating_add_unsigned(diff_top);
    placement.height = placement.height.saturating_sub(diff_top);

    let mut data = data.split_off(placement.width as usize * diff_top as usize);
    let data_bottom = data.iter().rev();
    let diff_height = data_bottom
        .take_while(|value| **value == 0)
        .count()
        .checked_div(placement.width as usize)
        .unwrap_or_default();

    let diff_height = diff_height.try_into().unwrap_or(u32::MAX);
    placement.height = placement.height.saturating_sub(diff_height);

    data.truncate(placement.width as usize * placement.height as usize);

    let (data, placement) = trim_left((data, placement));
    let (data, placement) = trim_right((data, placement));

    (data, placement)
}

fn trim_left((mut data, mut placement): Image) -> Image {
    let data_left = (0..placement.width as usize)
        .flat_map(|n| data.iter().skip(n).step_by(placement.width as usize));

    let diff_left = data_left
        .take_while(|value| **value == 0)
        .count()
        .checked_div(placement.height as usize)
        .unwrap_or_default();

    let diff_left = diff_left.try_into().unwrap_or(u32::MAX);
    let diff_left = diff_left.min(placement.width);
    placement.left = placement.left.saturating_add_unsigned(diff_left);

    'trim_op: for _ in 0..diff_left {
        if let Some(width) = placement.width.checked_sub(1) {
            for index in (0..placement.height as usize).map(|n| n * width as usize) {
                let _ = data.remove(index);
            }

            placement.width = width;
        } else {
            break 'trim_op;
        }
    }

    (data, placement)
}

fn trim_right((mut data, mut placement): Image) -> Image {
    let data_right = (0..placement.width as usize)
        .flat_map(|n| data.iter().rev().skip(n).step_by(placement.width as usize));

    let diff_width = data_right
        .take_while(|value| **value == 0)
        .count()
        .checked_div(placement.height as usize)
        .unwrap_or_default();

    let diff_width = diff_width.try_into().unwrap_or(u32::MAX);

    'trim_op: for _ in 0..diff_width {
        if let Some(width) = placement.width.checked_sub(1) {
            let rows = 0..placement.height as usize;
            for index in rows.map(|n| (n + 1) * width as usize) {
                let _ = data.remove(index);
            }

            placement.width = width;
        } else {
            break 'trim_op;
        }
    }

    (data, placement)
}
