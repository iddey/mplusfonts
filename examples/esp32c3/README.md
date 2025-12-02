## Prerequisites

For running the examples, make sure you have your environment set up for `ESP32-C3` development. The
compilers from `rustup` and `espup` installations both work.

- `stable` channel Rust with `riscv32imc-unknown-none-elf` added as a target or `esp` toolchain with
  `esp32c3` enabled as a target for builds.
- `espflash` tool, which is configured as the runner in `.cargo/config.toml` for uploading.

## Examples

Binaries are uploaded via USB in bootloader mode. Usage:
`cargo run --bin <example> --features esp32c3`

- `mini-mono` - An example that uses a `BinaryColor` bitmap font for a `BinaryColor` OLED display.
  - `common` - A library crate that contains the bitmap font definition used: `BITMAP_FONT_1`; this
    avoids having to do `mplus!` macro expansion with incremental builds of the example application
    crate.

The `esp32c3` feature is required for all binaries in this crate, it enables the `esp32c3` features
of the `esp-*` crates; this enables additional crate features for when a single processor core is
available.

## Minimum supported Rust version

The minimum supported Rust version for `mplusfonts-examples-esp32c3` is `1.89`.

## License

The source code of `mplusfonts-examples-esp32c3` is dual-licensed under:

* Apache License, Version 2.0 ([LICENSE-APACHE] or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License ([LICENSE-MIT] or <http://opensource.org/licenses/MIT>)

at your option.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
