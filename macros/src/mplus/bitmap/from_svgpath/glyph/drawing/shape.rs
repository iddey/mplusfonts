pub mod braille;

use swash::zeno::{Fill, Mask, PathData, Point};

use super::{GlyphMetrics, Image, ImageCluster};

type Points = [[Point; 25]; 25];

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

fn render_image(data: impl PathData, fill: Fill) -> Image {
    Mask::new(data).style(fill).render()
}
