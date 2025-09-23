use swash::zeno::{Fill, Placement, Vector};

use crate::mplus::bitmap::units::Grid;

use super::parts::*;
use super::tall::parts::*;
use super::wide::parts::*;
use super::{GlyphMetrics, ImageCluster};
use super::{render_image, render_image_shaded};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident,
            $fn_call_path_min_block:path,
            $fn_call_path_min_image:path,
            $fn_call_path_sub_block:path,
            $fn_call_path_sub_image:path,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.block;
                let min_block = $fn_call_path_min_block(points, offset);
                let (mut data, placement) = $fn_call_path_min_image(&min_block, Fill::NonZero);
                let sub_block = $fn_call_path_sub_block(points, offset);
                let (sub_data, sub_placement) = $fn_call_path_sub_image(&sub_block, Fill::NonZero);

                let Placement { top, left, width, .. } = sub_placement;
                let diff_left = left.saturating_sub(placement.left);
                let diff_left: usize = diff_left.try_into().expect("unexpected left overhang");
                let diff_top = top.saturating_sub(placement.top);
                let diff_top: usize = diff_top.try_into().expect("unexpected top overhang");
                let mut sub_values = sub_data.into_iter();
                assert!(placement.width.checked_sub(width).is_some(), "unexpected right overhang");

                'sub_op: for row_data in data.chunks_mut(placement.width as usize).skip(diff_top) {
                    for value in row_data.iter_mut().skip(diff_left).take(width as usize) {
                        let Some(sub_value) = sub_values.next() else {
                            break 'sub_op;
                        };

                        *value -= sub_value;
                    }
                }

                assert!(sub_values.next().is_none(), "unexpected bottom overhang");

                Vec::from([(data, placement)])
            }
        )*
    }
}

def_unicode_char! {
    inverse_medium_shade,
        full_block, render_image, full_block, render_image_shaded::<31>,

    upper_half_block_and_lower_half_inverse_medium_shade,
        full_block, render_image, lower_half_block, render_image_shaded::<31>,

    upper_half_inverse_medium_shade_and_lower_half_block,
        full_block, render_image, upper_half_block, render_image_shaded::<31>,

    left_half_inverse_medium_shade_and_right_half_block,
        full_block, render_image, left_half_block, render_image_shaded::<31>,
}
