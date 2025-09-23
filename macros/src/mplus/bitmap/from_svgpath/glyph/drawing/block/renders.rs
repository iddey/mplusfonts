use swash::zeno::{Fill, Vector};

use super::parts;
use super::render_image;
use super::{Image, Points};

pub fn full_block(points: &Points, offset: Vector) -> Image {
    let block = parts::full_block(points, offset);

    render_image(&block, Fill::NonZero)
}
