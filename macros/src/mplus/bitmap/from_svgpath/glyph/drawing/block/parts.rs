use swash::zeno::Vector;

use super::{Block, MapIndex, Points};

pub fn full_block(points: &Points, offset: Vector) -> Block {
    let index = Points::map_index::<1>(1);

    Block(points[0][0] + offset, points[index][index] + offset)
}
