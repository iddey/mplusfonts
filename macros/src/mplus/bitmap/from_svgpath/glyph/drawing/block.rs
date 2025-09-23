mod complex;
mod inverse;
mod misc;
mod octant;
mod parts;
mod renders;
mod sextant;
mod shade;
mod tall;
mod wide;

use std::{array, iter, slice};

use swash::zeno::{Command, Fill, Mask, PathData, Point, Vector};

use crate::mplus::bitmap::units::Grid;

use super::{GlyphMetrics, Image, ImageCluster};

pub use complex::*;
pub use inverse::*;
pub use misc::*;
pub use octant::*;
pub use sextant::*;
pub use shade::*;
pub use tall::*;
pub use wide::*;

type Points = [[Point; 25]; 25];
type Map<'a> = fn(&'a Block) -> array::IntoIter<Command, 5>;

pub struct Block(pub Point, pub Point);
pub struct BlockList(pub Vec<Block>);

trait MapIndex {
    fn map_index<const MAX: usize>(index: usize) -> usize;
}

impl<const N: usize> MapIndex for [[Point; N]; N] {
    fn map_index<const MAX: usize>(index: usize) -> usize {
        let 1.. = N else {
            panic!("expected `N` greater than `0`, found `{N}`")
        };
        let Some(step) = (N - 1).checked_div(MAX) else {
            panic!("expected `MAX` greater than `0`, found `{MAX}`");
        };

        index * step
    }
}

impl PathData for &Block {
    type Commands = array::IntoIter<Command, 5>;

    fn commands(&self) -> Self::Commands {
        let Block(Point { x: x0, y: y0 }, Point { x: x1, y: y1 }) = self;
        let commands = [
            Command::MoveTo(Point::new(*x0, *y0)),
            Command::LineTo(Point::new(*x1, *y0)),
            Command::LineTo(Point::new(*x1, *y1)),
            Command::LineTo(Point::new(*x0, *y1)),
            Command::Close,
        ];

        commands.into_iter()
    }
}

impl<'a> PathData for &'a BlockList {
    type Commands =
        iter::FlatMap<slice::Iter<'a, Block>, <&'a Block as PathData>::Commands, Map<'a>>;

    fn commands(&self) -> Self::Commands {
        let BlockList(blocks) = self;
        let map: Map = |block| block.commands();

        blocks.iter().flat_map(map)
    }
}

impl<T: Into<Vec<Block>>> From<T> for BlockList {
    fn from(chains: T) -> Self {
        Self(chains.into())
    }
}

fn render_image(data: impl PathData, fill: Fill) -> Image {
    Mask::new(data).style(fill).render()
}

fn render_image_shaded<const MAX: u8>(data: impl PathData, fill: Fill) -> Image {
    let (mut data, placement) = Mask::new(data).style(fill).render();
    for value in data.iter_mut() {
        *value = scale_back::<MAX>(*value);
    }

    (data, placement)
}

const fn scale_back<const MAX: u8>(value: u8) -> u8 {
    let result = MAX as i32 * value as i32 + 255;

    (result >> 8) as u8
}

pub fn full_block(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
    let Grid(points) = &glyph_metrics.block;

    Vec::from([renders::full_block(points, offset)])
}
