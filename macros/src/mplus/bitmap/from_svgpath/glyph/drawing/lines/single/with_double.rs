use swash::zeno::Vector;

use crate::mplus::bitmap::units::Grid;

use super::super::double::parts::*;
use super::super::single::parts::*;
use super::{ChainList, GlyphMetrics, ImageCluster};
use super::{offset_table, render_image};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident,
            $($fn_call_path:path,)?
            [$fn_call_path_to_double:path $(, $fn_call_path_to_single:path)? $(,)?],
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let offset = offset_table(offset);
                let (Grid(points), stroke) = &glyph_metrics.light;
                let mut chains = Vec::new();
                chains.extend($fn_call_path_to_double(points, offset, stroke));
                $(chains.push($fn_call_path_to_single(points, offset, stroke));)?
                $(chains.push($fn_call_path(points, offset));)?

                let chains = ChainList::from(chains);
                let image = render_image(&chains, stroke);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    down_single_and_right_double, [double_right, long_down],
    down_double_and_right_single, [double_down, long_right],
    down_single_and_left_double, [double_left, long_down],
    down_double_and_left_single, [double_down, long_left],

    up_single_and_right_double, [double_right, long_up],
    up_double_and_right_single, [double_up, long_right],
    up_single_and_left_double, [double_left, long_up],
    up_double_and_left_single, [double_up, long_left],

    vertical_single_and_right_double, vertical, [double_right],
    vertical_double_and_right_single, [double_vertical, short_right],
    vertical_single_and_left_double, vertical, [double_left],
    vertical_double_and_left_single, [double_vertical, short_left],

    down_single_and_horizontal_double, [double_horizontal, short_down],
    down_double_and_horizontal_single, horizontal, [double_down],
    up_single_and_horizontal_double, [double_horizontal, short_up],
    up_double_and_horizontal_single, horizontal, [double_up],

    vertical_single_and_horizontal_double, vertical, [double_horizontal],
    vertical_double_and_horizontal_single, horizontal, [double_vertical],
}
