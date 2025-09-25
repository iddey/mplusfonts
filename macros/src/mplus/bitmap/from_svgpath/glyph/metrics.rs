use swash::zeno::{Cap, Stroke};

use crate::mplus::bitmap::units::{Grid, Halfwidth};
use crate::mplus::font::Font;

pub struct GlyphMetrics<'a> {
    pub halfwidth: Halfwidth,
    pub is_code: bool,
    pub width: f32,
    pub top: f32,
    pub light: (Grid<3>, Stroke<'a>),
    pub heavy: (Grid<3>, Stroke<'a>),
    pub cross: (Grid<3>, Stroke<'a>),
    pub block: Grid<25>,
    pub shape: Grid<25>,
}

impl GlyphMetrics<'_> {
    pub fn from_font(font: &Font, pixels_per_em: f32, hint: bool) -> Self {
        let halfwidth = Halfwidth::from_font(font, pixels_per_em);
        let is_code = matches!(font, Font::MPLUSCode { .. });
        let width = match halfwidth {
            Halfwidth::Floor(halfwidth) if is_code => halfwidth.floor(),
            Halfwidth::Floor(halfwidth) => halfwidth,
            Halfwidth::Ceil => 1.0,
            Halfwidth::Zero if is_code => 0.0,
            Halfwidth::Zero => 0.5,
        };

        let top = pixels_per_em * if is_code { 1.235 } else { 1.16 };
        let top = top.ceil();
        let bottom = pixels_per_em * if is_code { -0.27 } else { -0.288 };
        let bottom = bottom.ceil();
        let height = match halfwidth {
            Halfwidth::Floor(_) => top - bottom,
            Halfwidth::Ceil => 2.0,
            Halfwidth::Zero if is_code => 0.0,
            Halfwidth::Zero => 1.0,
        };

        let thin_width = pixels_per_em * 0.05;
        let thin_width = if hint { thin_width.ceil() } else { thin_width };
        let thin_width_snap = hint.then_some((thin_width % 2.0 / 2.0, thin_width % 2.0 / 2.0));
        let thin_width_snap_grid = Grid::new(width, height, thin_width_snap);
        let mut thin_stroke = Stroke::new(thin_width);
        thin_stroke.cap(Cap::Square);

        let thick_width = 2.0 * thin_width;
        let thick_width_snap = hint.then_some((thick_width % 2.0 / 2.0, thick_width % 2.0 / 2.0));
        let thick_width_snap_grid = Grid::new(width, height, thick_width_snap);
        let mut thick_stroke = Stroke::new(thick_width);
        thick_stroke.cap(Cap::Square);

        let thin_width_no_snap_grid = Grid::new(width, height, None);
        let zero_width_snap = hint.then_some((0.0, 0.0));
        let zero_width_snap_grid = Grid::new(width, height, zero_width_snap);
        let half_pixel_width_snap = hint.then_some((0.5, 0.5));
        let half_pixel_width_snap_grid = Grid::new(width, height, half_pixel_width_snap);

        Self {
            halfwidth,
            is_code,
            width,
            top,
            light: (thin_width_snap_grid, thin_stroke),
            heavy: (thick_width_snap_grid, thick_stroke),
            cross: (thin_width_no_snap_grid, thin_stroke),
            block: zero_width_snap_grid,
            shape: half_pixel_width_snap_grid,
        }
    }
}
