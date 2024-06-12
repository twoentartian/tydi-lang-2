# Tydi-lang compiler v2

Compile Tydi-lang source code to a json representation of the typed streaming hardware. The generated json file can be further transformed to [Chisel](https://www.chisel-lang.org/) with [Tydi-Chisel
 tool](https://github.com/abs-tudelft/Tydi-Chisel).

## Build

1. Install [Rust](https://www.rust-lang.org/tools/install) toolchain.
2. Run `cargo build` in the folder which contains `Cargo.toml`.
3. Go to `target/debug` to find the binary file `tydi-lang-complier`.
4. Type `tydi-lang-complier -h` to see available commands.

## Syntax

Tydi-lang syntax file is [here](./tydi-lang2-syntax.md)

## Relative papers

- [Tydi specification](https://ieeexplore.ieee.org/abstract/document/9098092)
- [Tydi-lang](https://dl.acm.org/doi/fullHtml/10.1145/3624062.3624539) 