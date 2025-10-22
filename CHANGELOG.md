# Changelog

## [Unreleased]

### Fixed

- Drawing certain block elements (for example, `RIGHT ONE EIGHTH BLOCK`) for bitmap fonts that have
  certain `size` parameters, when some but not all of the _x_-offset parameters generate images that
  contain visible pixels, do no longer result in an invalid number of array elements if `positions`,
  greater than one, were expected.

### Changed

- Upgrade dev-dependencies: `embedded-text 0.7.3` and `embedded-graphics-simulator 0.8.0`.
- The same optimization that was applied to font rasterization, that allowed text shaping to become
  multithreaded, has been been applied to characters that are backed by vector graphics (box-drawing
  characters, block elements, and so on).

## [0.3.0] - 2025-10-08

### Added

- [`TextBox`] interprets ANSI escape codes for text styling with its ANSI plugin, and `mplusfonts`’s
  [`BitmapFontStyle`] now handles calls to set or reset the text and background colors, to enable or
  disable text underline, and to enable or disable strikethrough _programmatically_ as text is being
  rendered.

### Fixed

- Monospaced text using [`TextBox`] having its rightmost column of pixels cut off before whitespace
  for `code(100)` together with certain `size` parameters (for example, at 12 pixels per _em_-size).

[`TextBox`]: https://docs.rs/embedded-text/latest/embedded_text/struct.TextBox.html
[`BitmapFontStyle`]: https://docs.rs/mplusfonts/latest/mplusfonts/style/struct.BitmapFontStyle.html

### Changed

- Upgrade dependencies: `swash 0.2.6`.

## [0.2.4] - 2025-09-26

### Added

- Block elements, including shade and fill characters, for use in monospaced text.
- Block mosaic characters, which are two sets of semigraphical characters.
- A set of braille patterns with or without unpunched dots shown. See the `mplusfonts-macros` crate
  and its `alt-braille` feature for details.

### Changed

- Upgrade dependencies: `regex 1.11.3`.

## [0.2.3] - 2025-09-08

### Added

- Box-drawing characters with hinting available for use in monospaced text.

### Changed

- Upgrade dependencies: `regex 1.11.2` and `swash 0.2.5`.

## [0.2.2] - 2025-08-17

### Fixed

- The const generic parameter `N` in `NextGlyph<'_, _, N>` is now always equal to `positions`, even
  if there is only one glyph image. This fixes monospaced fonts that include diacritical marks when
  `positions` is greater than one.
- The relative positions of diacritical marks for decomposed characters. Their _y_-offsets are also
  set correctly for font rasterization and rendering. Note that the use of `hint` results in visual
  dissimilarity to diacritical marks in precomposed characters.

## [0.2.1] - 2025-08-06

### Fixed

- Japanese monospaced text out of alignment. All `code(width)` and `size` combinations now have the
  positions of glyph images adjusted so that they would appear center-aligned in a grid with single
  and double-width character cells.

## [0.2.0] - 2025-05-21

### Added

- Underline and strikethrough decorations for text. The interface of the style builder provides the
  necessary methods for enabling these graphics with various color options.

### Fixed

- All one-pixel-wide images at non-zero offsets being discarded, not only of whitespace characters.
- Background not filled in where two glyph images would overlap along the _x_-axis but are actually
  spaced out along the _y_-axis. The empty pixels are no longer there.

## [0.1.5] - 2025-05-09

### Fixed

- The font metrics to match their definitions in the TrueType fonts. The `ascender` is not used but
  is a public field.
- Whitespace character rendering changed in `swash 0.2.4`; zero-width glyph images are now handled.

### Changed

- The `strings` attribute macro to visit token streams when gathering string literals. For example,
  parameters to any macro such as [`format!`] and [`concat!`] are token streams.
- Upgrade dependencies: `swash 0.2.4`.

[`format!`]: https://doc.rust-lang.org/std/macro.format.html
[`concat!`]: https://doc.rust-lang.org/core/macro.concat.html

## [0.1.4] - 2025-04-17

### Changed

- Improve `mplus!` macro expansion performance for when the `kern` helper is used.
- Implementation of font rasterization to be multithreaded; this can only have a noticeable effect
  when `positions` is greater than one.

## [0.1.3] - 2025-04-03

### Added

- The `kern` helper that can be used to create variable-width bitmap fonts with font-based kerning
  when populated using character ranges. This was previously only possible when specifying strings.

### Changed

- Upgrade dependencies: `defmt 1.0` and `swash 0.2.2`.

## [0.1.2] - 2025-03-28

### Fixed

- Rust version `1.86` not compiling the `mplusfonts` crate.

## [0.1.1] - 2025-03-21

### Fixed

- Standard ligatures such as _ff_ and _ffi_ appearing in monospaced text; this is now disabled.

## [0.1.0] - 2025-03-20

### Added

- The `strings` attribute macro and its `strings::skip` and `strings::emit` helper attributes.
- The `mplus!` function-like procedural macro for bitmap font generation using the `swash` crate.
- Implementation of the text renderer interface from the `embedded-graphics` crate.
- Settings for the text and background colors.
- Fonts by Coji Morishita.

[0.1.0]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.0
[0.1.1]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.1
[0.1.2]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.2
[0.1.3]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.3
[0.1.4]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.4
[0.1.5]: https://github.com/iddey/mplusfonts/releases/tag/v0.1.5
[0.2.0]: https://github.com/iddey/mplusfonts/releases/tag/v0.2.0
[0.2.1]: https://github.com/iddey/mplusfonts/releases/tag/v0.2.1
[0.2.2]: https://github.com/iddey/mplusfonts/releases/tag/v0.2.2
[0.2.3]: https://github.com/iddey/mplusfonts/releases/tag/v0.2.3
[0.2.4]: https://github.com/iddey/mplusfonts/releases/tag/v0.2.4
[0.3.0]: https://github.com/iddey/mplusfonts/releases/tag/v0.3.0
