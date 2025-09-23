use swash::zeno::{Fill, Vector};

use crate::mplus::bitmap::units::Grid;

use super::parts::*;
use super::render_image_shaded;
use super::tall::parts::*;
use super::wide::parts::*;
use super::{GlyphMetrics, ImageCluster};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, $fn_call_path_block:path, $fn_call_path_image:path,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.block;
                let block = $fn_call_path_block(points, offset);
                let image = $fn_call_path_image(&block, Fill::NonZero);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    light_shade, full_block, render_image_shaded::<64>,
    medium_shade, full_block, render_image_shaded::<128>,
    dark_shade, full_block, render_image_shaded::<192>,

    left_half_medium_shade, left_half_block, render_image_shaded::<128>,
    right_half_medium_shade, right_half_block, render_image_shaded::<128>,
    upper_half_medium_shade, upper_half_block, render_image_shaded::<128>,
    lower_half_medium_shade, lower_half_block, render_image_shaded::<128>,
}
