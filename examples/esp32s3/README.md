## Prerequisites

For running the examples, make sure your have an environment set up for `ESP32-S3` development. The
compiler from an `espup` installation works.

- `esp` toolchain, which includes Rust with Xtensa support, with `esp32s3` enabled as a target.
- `espflash` tool, which is configured as the runner in `.cargo/config.toml` for uploading.

## Examples

Binaries are uploaded via USB in bootloader mode. Usage: `cargo run --bin <example>`

- `mini-mono` - An example that uses a `BinaryColor` bitmap font for a `BinaryColor` OLED display.
  - `common` - A library crate that contains the bitmap font definition used: `BITMAP_FONT_1`; this
    avoids having to do `mplus!` macro expansion with incremental builds of the example application
    crate.

## Minimum supported Rust version

The minimum supported Rust version for `mplusfonts-examples-esp32s3` is `1.89`.

## License

The source code of `mplusfonts-examples-esp32s3` is dual-licensed under:

* Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
