use swash::zeno::{Angle, ArcSize, ArcSweep, Command, PathBuilder, PathData, Vector};

use crate::mplus::bitmap::units::Grid;

use super::super::single::parts::*;
use super::{Chain, GlyphMetrics, ImageCluster};
use super::{offset_table, render_image, trim_image};

fn path_commands(chain: &Chain) -> Vec<Command> {
    let Chain(points) = chain;
    if let [first, _, last] = *points.as_slice() {
        let diff = last - first;
        let radius = f32::min(diff.x.abs(), diff.y.abs());
        let second = Vector::new(first.x, last.y - radius.copysign(diff.y));
        let third = Vector::new(first.x + radius.copysign(diff.x), last.y);
        let angle = Angle::from_degrees(90.0);
        let size = ArcSize::Small;
        let sweep = if diff.x.signum() * diff.y.signum() < 0.0 {
            ArcSweep::Positive
        } else {
            ArcSweep::Negative
        };

        let mut path = Vec::new();
        path.move_to(first);
        path.line_to(second);
        path.arc_to(radius, radius, angle, size, sweep, third);
        path.line_to(last);

        path
    } else {
        chain.commands().collect()
    }
}

macro_rules! def_unicode_char {
    (
        $(
            $fn_ident:ident, $fn_call_path:path,
        )*
    ) => {
        $(
            pub fn $fn_ident(glyph_metrics: &GlyphMetrics, offset: Vector) -> ImageCluster {
                let offset = offset_table(offset);
                let (Grid(points), stroke) = &glyph_metrics.light;
                let chain = $fn_call_path(points, offset);
                let commands = path_commands(&chain);
                let image = render_image(commands.as_slice(), stroke);
                let image = trim_image(image);

                Vec::from([image])
            }
        )*
    }
}

def_unicode_char! {
    light_arc_down_and_right, down_and_right,
    light_arc_down_and_left, down_and_left,

    light_arc_up_and_right, up_and_right,
    light_arc_up_and_left, up_and_left,
}
