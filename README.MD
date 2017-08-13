# rfyl - An RPG dice roller.

A dice roller accepting dice notation written in Rust. A spiritual successor to [froll](https://github.com/trnglina/froll).

[Documentation](https://trnglina.org/doc/rfyl)

[Also on crates.io if you want](https://crates.io/crates/rfyl).

## Supported input

- Basic standard dice notation: `d8`, `2d12`.
- Addition: `d4 + 2d6`.
- Subtraction: `d100 - 15`.
- Multiplication: `d12 * 2`.
- Division: `d100 / 15`. (Note that fractional values are rounded to the nearest integer.)
- Brackets: `(d100 + d12) / 15`.
- Complex dice notation: `1d4 + 2d6 * 3d2 / 4d8 + (2d6 + 3d8) - 16 * (1 / 1d4)`.
- [master] Percentile dice shorthand: `d%` = `d100`.

## License

rfyl is Public Domain software available under the Creative Commons 0. See [LICENSE](LICENSE) and [COPYING](COPYING) for more information.
