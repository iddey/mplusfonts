## Prerequisites

For running the examples, make sure your have an environment set up for `RP2040` development. The
compiler from a `rustup` installation works.

- `stable` channel Rust with `thumbv6m-none-eabi` added as a target for builds.
- `flip-link` is configured as the linker in `.cargo/config.toml`.
- `picotool 2.2.0` or newer as the runner for uploading.

## Examples

Binaries are uploaded via USB in bootloader mode. Usage: `cargo run --bin <example>`

- `mini-mono` - An example that uses a `BinaryColor` bitmap font for a `BinaryColor` OLED display.
  - `common` - A library crate that contains the bitmap font definition used: `BITMAP_FONT_1`; this
    avoids having to do `mplus!` macro expansion with incremental builds of the example application
    crate.

## Minimum supported Rust version

The minimum supported Rust version for `mplusfonts-examples-rp2040` is `1.89`.

## License

The source code of `mplusfonts-examples-rp2040` is dual-licensed under:

* Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
