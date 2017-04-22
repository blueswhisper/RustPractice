# RustPractice
Rust snippt.

This repo record some rust source code when I learn rust follow [Rust By Example Chinese Version](https://rustwiki.org/rust-by-example/index.html).

# Quick Start

- Run by [Cargo](https://github.com/rust-lang/cargo)
    + I suppose you have `Cargo` installed
    + `cd RustPractice`
    + `cargo run --example ${example}`
    + eg: `cargo run --example hello`, you can find all example in `Cargo.toml`-> `[[example]]` section

- Run by [Rust](https://www.rust-lang.org)
    + I suppose you have `Rust compiler` installed
    + `cd RustPractice/examples/`
    + `rustc ${example file}`
    + `./${example}`
    + eg:
    
        ```bash
        cd RustPractice/examples/
        rustc format.rs
        ./format
        Dublin: 53.348°N 6.260°W
        Oslo: 59.950°N 10.750°E
        Vancouver: 49.250°N 123.100°W
        Color { red: 128, green: 255, blue: 90 }
        RGB (128, 255, 90) 0x80FF5A
        Color { red: 0, green: 3, blue: 254 }
        RGB (0, 3, 254) 0x0003FE
        Color { red: 0, green: 0, blue: 0 }
        RGB (0, 0, 0) 0x000000
        ```