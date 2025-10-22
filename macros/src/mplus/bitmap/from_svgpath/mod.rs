mod glyph;

use std::collections::BTreeMap;
use std::iter;
use std::sync::RwLock;
use std::thread;

use glyph::{GlyphDrawing, GlyphMetrics};

use crate::mplus::Arguments;
use crate::mplus::bitmap::CharDictionary;
use crate::mplus::charmap::CharmapEntry;

pub fn render(args: &Arguments) -> BTreeMap<String, CharmapEntry> {
    let entries = BTreeMap::new();
    let font = args.font.value();
    let pixels_per_em = args.size.into_value();
    let hint = args.hint.into_value();
    let glyph_metrics = GlyphMetrics::from_font(font, pixels_per_em, hint);
    let positions = args.positions.into_value();
    let bit_depth = args.bit_depth.into_value();
    let strings = args.sources.iter().flat_map(|source| source.strings(false));
    let indices = 0..thread::available_parallelism().map(Into::into).unwrap_or(1);
    let mut glyph_drawings: Vec<_> = iter::repeat_with(Vec::new).take(indices.end).collect();
    let mut indices = indices.cycle();
    for strings in strings {
        strings
            .iter()
            .flat_map(|string| string.chars())
            .filter_map(GlyphDrawing::get)
            .zip(iter::repeat_with(|| indices.next().unwrap_or_default()))
            .for_each(|(glyph_drawing, index)| glyph_drawings[index].push(glyph_drawing));
    }

    let entries = RwLock::new(entries);
    thread::scope(|scope| {
        let entries = CharDictionary::new(&entries);
        let glyph_metrics = &glyph_metrics;
        for glyph_drawings in glyph_drawings {
            scope.spawn(move || {
                for glyph_drawing in glyph_drawings {
                    let entry_key = glyph_drawing.key.into();
                    if !entries.contains_key(&entry_key) {
                        let glyphs = glyph_drawing.scale(positions, bit_depth, glyph_metrics);
                        entries.insert_glyphs(entry_key, glyphs);
                    }
                }
            });
        }
    });

    entries
        .into_inner()
        .expect("expected no-poison lock on entries")
}
