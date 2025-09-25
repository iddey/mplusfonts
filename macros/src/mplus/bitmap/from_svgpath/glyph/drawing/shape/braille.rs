use std::array;

use swash::zeno::{Fill, PathBuilder, Vector};

use crate::mplus::bitmap::units::Grid;

use super::render_image;
use super::{GlyphMetrics, ImageCluster, MapIndex, Points};

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, [$($dot_number:expr),*] $(- $dot_number_one:expr)?,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let Grid(points) = &glyph_metrics.shape;
                let dot_numbers = [$($dot_number),*];
                let dot_number_one = 0 $(+ $dot_number_one)?;

                let mut avg_diff_x_abs = 0f32;
                let mut avg_diff_y_abs = 0f32;
                let mut avg_item_count = 0usize;
                let center_points: [_; 8] = array::from_fn(|index| {
                    let x0_index = index % 2;
                    let x1_index = x0_index + 1;
                    let y0_index = index / 2;
                    let y1_index = y0_index + 1;

                    let x0_index = Points::map_index::<2>(x0_index);
                    let x1_index = Points::map_index::<2>(x1_index);
                    let y0_index = Points::map_index::<4>(y0_index);
                    let y1_index = Points::map_index::<4>(y1_index);

                    let center_x_index = usize::midpoint(x0_index, x1_index);
                    let center_y_index = usize::midpoint(y0_index, y1_index);
                    let center_point = points[center_y_index][center_x_index] + offset;
                    let corner_point = points[y0_index][x0_index] + offset;

                    let diff = center_point - corner_point;
                    let calc_sum_x = avg_diff_x_abs.mul_add(avg_item_count as f32, diff.x.abs());
                    let calc_sum_y = avg_diff_y_abs.mul_add(avg_item_count as f32, diff.y.abs());
                    avg_item_count += 1;
                    avg_diff_x_abs = calc_sum_x / avg_item_count as f32;
                    avg_diff_y_abs = calc_sum_y / avg_item_count as f32;

                    center_point
                });
                let radius = f32::min(avg_diff_x_abs, avg_diff_y_abs);
                let radius = 2.0 * radius.max(1.0) / 3.0;
                let width = 2.0 * radius;

                let mut path = Vec::new();
                for (index, center_point) in center_points.into_iter().enumerate() {
                    let dot_number = dot_number_one + match index {
                        1 => 3,
                        2 => 1,
                        3 => 4,
                        4 => 2,
                        i => i,
                    };

                    if dot_numbers.contains(&dot_number) {
                        path.add_circle(center_point, radius);
                    } else if cfg!(feature = "alt-braille") {
                        let top_left = center_point - Vector::from(radius / 3.0);
                        path.add_rect(top_left, width / 3.0, width / 3.0);
                    }
                }

                let image = render_image(&path, Fill::NonZero);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    blank, [],

    dots_1, [1] - 1,
    dots_2, [2] - 1,
    dots_3, [3] - 1,
    dots_4, [4] - 1,
    dots_5, [5] - 1,
    dots_6, [6] - 1,
    dots_7, [7] - 1,
    dots_8, [8] - 1,

    dots_12, [1, 2] - 1,
    dots_13, [1, 3] - 1,
    dots_14, [1, 4] - 1,
    dots_15, [1, 5] - 1,
    dots_16, [1, 6] - 1,
    dots_17, [1, 7] - 1,
    dots_18, [1, 8] - 1,

    dots_23, [2, 3] - 1,
    dots_24, [2, 4] - 1,
    dots_25, [2, 5] - 1,
    dots_26, [2, 6] - 1,
    dots_27, [2, 7] - 1,
    dots_28, [2, 8] - 1,
    dots_34, [3, 4] - 1,
    dots_35, [3, 5] - 1,
    dots_36, [3, 6] - 1,
    dots_37, [3, 7] - 1,
    dots_38, [3, 8] - 1,
    dots_45, [4, 5] - 1,
    dots_46, [4, 6] - 1,
    dots_47, [4, 7] - 1,
    dots_48, [4, 8] - 1,
    dots_56, [5, 6] - 1,
    dots_57, [5, 7] - 1,
    dots_58, [5, 8] - 1,
    dots_67, [6, 7] - 1,
    dots_68, [6, 8] - 1,
    dots_78, [7, 8] - 1,

    dots_123, [1, 2, 3] - 1,
    dots_124, [1, 2, 4] - 1,
    dots_125, [1, 2, 5] - 1,
    dots_126, [1, 2, 6] - 1,
    dots_127, [1, 2, 7] - 1,
    dots_128, [1, 2, 8] - 1,
    dots_134, [1, 3, 4] - 1,
    dots_135, [1, 3, 5] - 1,
    dots_136, [1, 3, 6] - 1,
    dots_137, [1, 3, 7] - 1,
    dots_138, [1, 3, 8] - 1,
    dots_145, [1, 4, 5] - 1,
    dots_146, [1, 4, 6] - 1,
    dots_147, [1, 4, 7] - 1,
    dots_148, [1, 4, 8] - 1,
    dots_156, [1, 5, 6] - 1,
    dots_157, [1, 5, 7] - 1,
    dots_158, [1, 5, 8] - 1,
    dots_167, [1, 6, 7] - 1,
    dots_168, [1, 6, 8] - 1,
    dots_178, [1, 7, 8] - 1,

    dots_234, [2, 3, 4] - 1,
    dots_235, [2, 3, 5] - 1,
    dots_236, [2, 3, 6] - 1,
    dots_237, [2, 3, 7] - 1,
    dots_238, [2, 3, 8] - 1,
    dots_245, [2, 4, 5] - 1,
    dots_246, [2, 4, 6] - 1,
    dots_247, [2, 4, 7] - 1,
    dots_248, [2, 4, 8] - 1,
    dots_256, [2, 5, 6] - 1,
    dots_257, [2, 5, 7] - 1,
    dots_258, [2, 5, 8] - 1,
    dots_267, [2, 6, 7] - 1,
    dots_268, [2, 6, 8] - 1,
    dots_278, [2, 7, 8] - 1,
    dots_345, [3, 4, 5] - 1,
    dots_346, [3, 4, 6] - 1,
    dots_347, [3, 4, 7] - 1,
    dots_348, [3, 4, 8] - 1,
    dots_356, [3, 5, 6] - 1,
    dots_357, [3, 5, 7] - 1,
    dots_358, [3, 5, 8] - 1,
    dots_367, [3, 6, 7] - 1,
    dots_368, [3, 6, 8] - 1,
    dots_378, [3, 7, 8] - 1,
    dots_456, [4, 5, 6] - 1,
    dots_457, [4, 5, 7] - 1,
    dots_458, [4, 5, 8] - 1,
    dots_467, [4, 6, 7] - 1,
    dots_468, [4, 6, 8] - 1,
    dots_478, [4, 7, 8] - 1,
    dots_567, [5, 6, 7] - 1,
    dots_568, [5, 6, 8] - 1,
    dots_578, [5, 7, 8] - 1,
    dots_678, [6, 7, 8] - 1,

    dots_1234, [1, 2, 3, 4] - 1,
    dots_1235, [1, 2, 3, 5] - 1,
    dots_1236, [1, 2, 3, 6] - 1,
    dots_1237, [1, 2, 3, 7] - 1,
    dots_1238, [1, 2, 3, 8] - 1,
    dots_1245, [1, 2, 4, 5] - 1,
    dots_1246, [1, 2, 4, 6] - 1,
    dots_1247, [1, 2, 4, 7] - 1,
    dots_1248, [1, 2, 4, 8] - 1,
    dots_1256, [1, 2, 5, 6] - 1,
    dots_1257, [1, 2, 5, 7] - 1,
    dots_1258, [1, 2, 5, 8] - 1,
    dots_1267, [1, 2, 6, 7] - 1,
    dots_1268, [1, 2, 6, 8] - 1,
    dots_1278, [1, 2, 7, 8] - 1,
    dots_1345, [1, 3, 4, 5] - 1,
    dots_1346, [1, 3, 4, 6] - 1,
    dots_1347, [1, 3, 4, 7] - 1,
    dots_1348, [1, 3, 4, 8] - 1,
    dots_1356, [1, 3, 5, 6] - 1,
    dots_1357, [1, 3, 5, 7] - 1,
    dots_1358, [1, 3, 5, 8] - 1,
    dots_1367, [1, 3, 6, 7] - 1,
    dots_1368, [1, 3, 6, 8] - 1,
    dots_1378, [1, 3, 7, 8] - 1,
    dots_1456, [1, 4, 5, 6] - 1,
    dots_1457, [1, 4, 5, 7] - 1,
    dots_1458, [1, 4, 5, 8] - 1,
    dots_1467, [1, 4, 6, 7] - 1,
    dots_1468, [1, 4, 6, 8] - 1,
    dots_1478, [1, 4, 7, 8] - 1,
    dots_1567, [1, 5, 6, 7] - 1,
    dots_1568, [1, 5, 6, 8] - 1,
    dots_1578, [1, 5, 7, 8] - 1,
    dots_1678, [1, 6, 7, 8] - 1,

    dots_2345, [2, 3, 4, 5] - 1,
    dots_2346, [2, 3, 4, 6] - 1,
    dots_2347, [2, 3, 4, 7] - 1,
    dots_2348, [2, 3, 4, 8] - 1,
    dots_2356, [2, 3, 5, 6] - 1,
    dots_2357, [2, 3, 5, 7] - 1,
    dots_2358, [2, 3, 5, 8] - 1,
    dots_2367, [2, 3, 6, 7] - 1,
    dots_2368, [2, 3, 6, 8] - 1,
    dots_2378, [2, 3, 7, 8] - 1,
    dots_2456, [2, 4, 5, 6] - 1,
    dots_2457, [2, 4, 5, 7] - 1,
    dots_2458, [2, 4, 5, 8] - 1,
    dots_2467, [2, 4, 6, 7] - 1,
    dots_2468, [2, 4, 6, 8] - 1,
    dots_2478, [2, 4, 7, 8] - 1,
    dots_2567, [2, 5, 6, 7] - 1,
    dots_2568, [2, 5, 6, 8] - 1,
    dots_2578, [2, 5, 7, 8] - 1,
    dots_2678, [2, 6, 7, 8] - 1,
    dots_3456, [3, 4, 5, 6] - 1,
    dots_3457, [3, 4, 5, 7] - 1,
    dots_3458, [3, 4, 5, 8] - 1,
    dots_3467, [3, 4, 6, 7] - 1,
    dots_3468, [3, 4, 6, 8] - 1,
    dots_3478, [3, 4, 7, 8] - 1,
    dots_3567, [3, 5, 6, 7] - 1,
    dots_3568, [3, 5, 6, 8] - 1,
    dots_3578, [3, 5, 7, 8] - 1,
    dots_3678, [3, 6, 7, 8] - 1,
    dots_4567, [4, 5, 6, 7] - 1,
    dots_4568, [4, 5, 6, 8] - 1,
    dots_4578, [4, 5, 7, 8] - 1,
    dots_4678, [4, 6, 7, 8] - 1,
    dots_5678, [5, 6, 7, 8] - 1,

    dots_12345, [1, 2, 3, 4, 5] - 1,
    dots_12346, [1, 2, 3, 4, 6] - 1,
    dots_12347, [1, 2, 3, 4, 7] - 1,
    dots_12348, [1, 2, 3, 4, 8] - 1,
    dots_12356, [1, 2, 3, 5, 6] - 1,
    dots_12357, [1, 2, 3, 5, 7] - 1,
    dots_12358, [1, 2, 3, 5, 8] - 1,
    dots_12367, [1, 2, 3, 6, 7] - 1,
    dots_12368, [1, 2, 3, 6, 8] - 1,
    dots_12378, [1, 2, 3, 7, 8] - 1,
    dots_12456, [1, 2, 4, 5, 6] - 1,
    dots_12457, [1, 2, 4, 5, 7] - 1,
    dots_12458, [1, 2, 4, 5, 8] - 1,
    dots_12467, [1, 2, 4, 6, 7] - 1,
    dots_12468, [1, 2, 4, 6, 8] - 1,
    dots_12478, [1, 2, 4, 7, 8] - 1,
    dots_12567, [1, 2, 5, 6, 7] - 1,
    dots_12568, [1, 2, 5, 6, 8] - 1,
    dots_12578, [1, 2, 5, 7, 8] - 1,
    dots_12678, [1, 2, 6, 7, 8] - 1,
    dots_13456, [1, 3, 4, 5, 6] - 1,
    dots_13457, [1, 3, 4, 5, 7] - 1,
    dots_13458, [1, 3, 4, 5, 8] - 1,
    dots_13467, [1, 3, 4, 6, 7] - 1,
    dots_13468, [1, 3, 4, 6, 8] - 1,
    dots_13478, [1, 3, 4, 7, 8] - 1,
    dots_13567, [1, 3, 5, 6, 7] - 1,
    dots_13568, [1, 3, 5, 6, 8] - 1,
    dots_13578, [1, 3, 5, 7, 8] - 1,
    dots_13678, [1, 3, 6, 7, 8] - 1,
    dots_14567, [1, 4, 5, 6, 7] - 1,
    dots_14568, [1, 4, 5, 6, 8] - 1,
    dots_14578, [1, 4, 5, 7, 8] - 1,
    dots_14678, [1, 4, 6, 7, 8] - 1,
    dots_15678, [1, 5, 6, 7, 8] - 1,

    dots_23456, [2, 3, 4, 5, 6] - 1,
    dots_23457, [2, 3, 4, 5, 7] - 1,
    dots_23458, [2, 3, 4, 5, 8] - 1,
    dots_23467, [2, 3, 4, 6, 7] - 1,
    dots_23468, [2, 3, 4, 6, 8] - 1,
    dots_23478, [2, 3, 4, 7, 8] - 1,
    dots_23567, [2, 3, 5, 6, 7] - 1,
    dots_23568, [2, 3, 5, 6, 8] - 1,
    dots_23578, [2, 3, 5, 7, 8] - 1,
    dots_23678, [2, 3, 6, 7, 8] - 1,
    dots_24567, [2, 4, 5, 6, 7] - 1,
    dots_24568, [2, 4, 5, 6, 8] - 1,
    dots_24578, [2, 4, 5, 7, 8] - 1,
    dots_24678, [2, 4, 6, 7, 8] - 1,
    dots_25678, [2, 5, 6, 7, 8] - 1,
    dots_34567, [3, 4, 5, 6, 7] - 1,
    dots_34568, [3, 4, 5, 6, 8] - 1,
    dots_34578, [3, 4, 5, 7, 8] - 1,
    dots_34678, [3, 4, 6, 7, 8] - 1,
    dots_35678, [3, 5, 6, 7, 8] - 1,
    dots_45678, [4, 5, 6, 7, 8] - 1,

    dots_123456, [1, 2, 3, 4, 5, 6] - 1,
    dots_123457, [1, 2, 3, 4, 5, 7] - 1,
    dots_123458, [1, 2, 3, 4, 5, 8] - 1,
    dots_123467, [1, 2, 3, 4, 6, 7] - 1,
    dots_123468, [1, 2, 3, 4, 6, 8] - 1,
    dots_123478, [1, 2, 3, 4, 7, 8] - 1,
    dots_123567, [1, 2, 3, 5, 6, 7] - 1,
    dots_123568, [1, 2, 3, 5, 6, 8] - 1,
    dots_123578, [1, 2, 3, 5, 7, 8] - 1,
    dots_123678, [1, 2, 3, 6, 7, 8] - 1,
    dots_124567, [1, 2, 4, 5, 6, 7] - 1,
    dots_124568, [1, 2, 4, 5, 6, 8] - 1,
    dots_124578, [1, 2, 4, 5, 7, 8] - 1,
    dots_124678, [1, 2, 4, 6, 7, 8] - 1,
    dots_125678, [1, 2, 5, 6, 7, 8] - 1,
    dots_134567, [1, 3, 4, 5, 6, 7] - 1,
    dots_134568, [1, 3, 4, 5, 6, 8] - 1,
    dots_134578, [1, 3, 4, 5, 7, 8] - 1,
    dots_134678, [1, 3, 4, 6, 7, 8] - 1,
    dots_135678, [1, 3, 5, 6, 7, 8] - 1,
    dots_145678, [1, 4, 5, 6, 7, 8] - 1,

    dots_234567, [2, 3, 4, 5, 6, 7] - 1,
    dots_234568, [2, 3, 4, 5, 6, 8] - 1,
    dots_234578, [2, 3, 4, 5, 7, 8] - 1,
    dots_234678, [2, 3, 4, 6, 7, 8] - 1,
    dots_235678, [2, 3, 5, 6, 7, 8] - 1,
    dots_245678, [2, 4, 5, 6, 7, 8] - 1,
    dots_345678, [3, 4, 5, 6, 7, 8] - 1,

    dots_1234567, [1, 2, 3, 4, 5, 6, 7] - 1,
    dots_1234568, [1, 2, 3, 4, 5, 6, 8] - 1,
    dots_1234578, [1, 2, 3, 4, 5, 7, 8] - 1,
    dots_1234678, [1, 2, 3, 4, 6, 7, 8] - 1,
    dots_1235678, [1, 2, 3, 5, 6, 7, 8] - 1,
    dots_1245678, [1, 2, 4, 5, 6, 7, 8] - 1,
    dots_1345678, [1, 3, 4, 5, 6, 7, 8] - 1,
    dots_2345678, [2, 3, 4, 5, 6, 7, 8] - 1,

    dots_12345678, [1, 2, 3, 4, 5, 6, 7, 8] - 1,
}
