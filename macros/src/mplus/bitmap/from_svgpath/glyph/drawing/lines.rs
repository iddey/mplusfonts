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
