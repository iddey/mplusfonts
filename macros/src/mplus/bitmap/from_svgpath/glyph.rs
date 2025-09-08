mod drawing;
mod metrics;

use swash::zeno::Vector;

use crate::mplus::bitmap::color;
use crate::mplus::bitmap::units::Halfwidth;
use crate::mplus::bitmap::{Glyph, Image, ImageList};

pub use drawing::GlyphDrawing;
pub use metrics::GlyphMetrics;

impl GlyphDrawing {
    pub fn scale(&self, positions: u8, bit_depth: u8, glyph_metrics: &GlyphMetrics) -> Vec<Glyph> {
        let advance_width = glyph_metrics.width;
        let is_repeating = glyph_metrics.is_code;
        let images = if let Halfwidth::Floor(_) | Halfwidth::Ceil = glyph_metrics.halfwidth {
            let length = if is_repeating { 1 } else { positions };
            let mut images = Vec::new();
            (0..length).for_each(|index| {
                let x_offset = f32::from(index) / f32::from(length);
                let render = self.render;
                let offset = Vector::new(x_offset, 0.0);
                let image_cluster = render(glyph_metrics, offset);
                for (image_index, (data, placement)) in image_cluster.into_iter().enumerate() {
                    let left = placement.left;
                    let top = glyph_metrics.top as i32 - placement.top;
                    let width = placement.width;
                    let data = color::quantize(&data, width, bit_depth);
                    let image = Image {
                        left,
                        top,
                        width,
                        data,
                    };

                    if index == 0 {
                        images.push(Vec::new());
                    }

                    images
                        .get_mut(image_index)
                        .expect("exected image index to be less than number of images at `0`")
                        .push(image);
                }
            });

            images
        } else {
            Vec::new()
        };

        let mut images = images.into_iter().zip(self.id..);
        let first = images.next().map(|(images, id)| Glyph {
            x_offset: 0.0,
            y_offset: 0.0,
            positions,
            bit_depth,
            id,
            advance_width,
            images: ImageList(images),
        });

        let x_offset = -advance_width;
        let rest = images.map(|(images, id)| Glyph {
            x_offset,
            y_offset: 0.0,
            positions,
            bit_depth,
            id,
            advance_width: 0.0,
            images: ImageList(images),
        });

        first.into_iter().chain(rest).collect()
    }
}
