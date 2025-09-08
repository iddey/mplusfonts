use swash::zeno::Stroke;

use crate::mplus::bitmap::units::Grid;

use super::parts;
use super::render_image;
use super::{Chain, GlyphMetrics, Image, Matrix, Points};

fn render_image_dashed<const N: usize>(chain: &Chain, stroke: &Stroke) -> Image {
    let Chain(points) = chain;
    if let [start, end] = *points.as_slice() {
        let line_length = start.distance_to(end);
        let dash_length = line_length / if N > 0 { 2.0 * N as f32 } else { 1.0 };
        let dashes = [dash_length; 2];
        let offset = -dash_length / 2.0;
        let mut stroke = Stroke::new(stroke.width);
        stroke.dash(&dashes, offset);

        render_image(chain, &stroke)
    } else {
        render_image(chain, stroke)
    }
}

fn horizontal(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::horizontal(points, offset);

    render_image(&chain, stroke)
}

fn vertical(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::vertical(points, offset);

    render_image(&chain, stroke)
}

fn horizontal_dashed<const N: usize>(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::horizontal(points, offset);

    render_image_dashed::<N>(&chain, stroke)
}

fn vertical_dashed<const N: usize>(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::vertical(points, offset);

    render_image_dashed::<N>(&chain, stroke)
}

fn left(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::left(points, offset);

    render_image(&chain, stroke)
}

fn up(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::up(points, offset);

    render_image(&chain, stroke)
}

fn right(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::right(points, offset);

    render_image(&chain, stroke)
}

fn down(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::down(points, offset);

    render_image(&chain, stroke)
}

fn down_and_right(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::down_and_right(points, offset);

    render_image(&chain, stroke)
}

fn down_and_left(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::down_and_left(points, offset);

    render_image(&chain, stroke)
}

fn up_and_right(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::up_and_right(points, offset);

    render_image(&chain, stroke)
}

fn up_and_left(points: &Points, offset: Matrix, stroke: &Stroke) -> Image {
    let chain = parts::up_and_left(points, offset);

    render_image(&chain, stroke)
}

macro_rules! def_light_and_heavy {
    (
        $(
            $fn_ident:ident,
            $fn_call_path:path,
            $glyph_metrics_ident:ident,
            $glyph_metrics_field:expr,
        )*
    ) => {
        $(
            pub fn $fn_ident($glyph_metrics_ident: &GlyphMetrics, offset: Matrix) -> Image {
                let (Grid(points), stroke) = $glyph_metrics_field;

                $fn_call_path(points, offset, stroke)
            }
        )*
    }
}

def_light_and_heavy! {
    light_horizontal, horizontal, glyph_metrics, &glyph_metrics.light,
    heavy_horizontal, horizontal, glyph_metrics, &glyph_metrics.heavy,
    light_vertical, vertical, glyph_metrics, &glyph_metrics.light,
    heavy_vertical, vertical, glyph_metrics, &glyph_metrics.heavy,

    light_double_dash_horizontal, horizontal_dashed::<2>, glyph_metrics, &glyph_metrics.light,
    heavy_double_dash_horizontal, horizontal_dashed::<2>, glyph_metrics, &glyph_metrics.heavy,
    light_double_dash_vertical, vertical_dashed::<2>, glyph_metrics, &glyph_metrics.light,
    heavy_double_dash_vertical, vertical_dashed::<2>, glyph_metrics, &glyph_metrics.heavy,

    light_triple_dash_horizontal, horizontal_dashed::<3>, glyph_metrics, &glyph_metrics.light,
    heavy_triple_dash_horizontal, horizontal_dashed::<3>, glyph_metrics, &glyph_metrics.heavy,
    light_triple_dash_vertical, vertical_dashed::<3>, glyph_metrics, &glyph_metrics.light,
    heavy_triple_dash_vertical, vertical_dashed::<3>, glyph_metrics, &glyph_metrics.heavy,

    light_quadruple_dash_horizontal, horizontal_dashed::<4>, glyph_metrics, &glyph_metrics.light,
    heavy_quadruple_dash_horizontal, horizontal_dashed::<4>, glyph_metrics, &glyph_metrics.heavy,
    light_quadruple_dash_vertical, vertical_dashed::<4>, glyph_metrics, &glyph_metrics.light,
    heavy_quadruple_dash_vertical, vertical_dashed::<4>, glyph_metrics, &glyph_metrics.heavy,

    light_left, left, glyph_metrics, &glyph_metrics.light,
    heavy_left, left, glyph_metrics, &glyph_metrics.heavy,
    light_up, up, glyph_metrics, &glyph_metrics.light,
    heavy_up, up, glyph_metrics, &glyph_metrics.heavy,

    light_right, right, glyph_metrics, &glyph_metrics.light,
    heavy_right, right, glyph_metrics, &glyph_metrics.heavy,
    light_down, down, glyph_metrics, &glyph_metrics.light,
    heavy_down, down, glyph_metrics, &glyph_metrics.heavy,

    light_down_and_right, down_and_right, glyph_metrics, &glyph_metrics.light,
    heavy_down_and_right, down_and_right, glyph_metrics, &glyph_metrics.heavy,
    light_down_and_left, down_and_left, glyph_metrics, &glyph_metrics.light,
    heavy_down_and_left, down_and_left, glyph_metrics, &glyph_metrics.heavy,

    light_up_and_right, up_and_right, glyph_metrics, &glyph_metrics.light,
    heavy_up_and_right, up_and_right, glyph_metrics, &glyph_metrics.heavy,
    light_up_and_left, up_and_left, glyph_metrics, &glyph_metrics.light,
    heavy_up_and_left, up_and_left, glyph_metrics, &glyph_metrics.heavy,
}
