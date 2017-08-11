# RFYL - An RPG dice roller.

A dice roller accepting dice notation written in Rust. A spiritual successor to [froll](https://github.com/trnglina/froll).

## Supported input

- Basic standard dice notation: `d8`, `2d12`.
- Addition: `d4 + 2d6`.
- Subtraction: `d100 - 15`.
- Multiplication: `d12 * 2`.
- Division: `d100 / 15`. (Note that fractional values are rounded to the nearest integer.)
- Brackets: `(d100 + d12) / 15`.
- Complex dice notation: `1d4 + 2d6 * 3d2 / 4d8 + (2d6 + 3d8) - 16 * (1 / 1d4)`.

## Documentation

Static functions

`pub fn roll(input: &str)` -> Returns a `DiceRolls` object.

Struct `DiceRolls`

| Fields          | Type            | Description                                                                                                                                 |
|-----------------|-----------------|---------------------------------------------------------------------------------------------------------------------------------------------|
| `rolls`         | `Vec<DiceRoll>` | Vector of `DiceRoll`.                                                                                                                       |
| `formula`       | `Vec<String>`   | Vector of `String`s that contains the parsed formula written in postfix (RPN).                                                              |
| `rolls_formula` | `Vec<String>`   | Vector of `String`s that contains the parsed formula written in postfix (RPN) with the original dice notation instead of the rolled result. |

| Methods                                           | Return Type | Description                                                                                                           |
|---------------------------------------------------|-------------|-----------------------------------------------------------------------------------------------------------------------|
| `pub fn get_result(&self)`                        | `i32`       | Returns the solved result of the formula.                                                                             |
| `pub fn get_sum_of_rolls(&self)`                  | `i32`       | Returns a simple sum of the rolls.                                                                                    |
| `pub fn get_rolls_string(&self)`                  | `String`    | Returns a formatted `String` showing the dice and the rolled results.                                                 |
| `pub fn get_formula_string_as_rpn(&self)`         | `String`    | Returns a postfix formatted `String` showing the formula.                                                             |
| `pub fn get_formula_string_as_infix(&self)`       | `String`    | Returns an infix formatted `String` showing the formula.                                                              |
| `pub fn get_rolls_formula_string_as_rpn(&self)`   | `String`    | Returns a postfix formatted `String` showing the formula withthe original dice notation instead of the rolled result. |
| `pub fn get_rolls_formula_string_as_infix(&self)` | `String`    | Returns a infix formatted `String` showing the formula withthe original dice notation instead of the rolled result.   |

Struct `DiceRoll`

| Fields          | Type    | Description                           |
|-----------------|---------|---------------------------------------|
| `sides`         | `i32`   | Number of sides the rolled dice had   |
| `result`        | `i32`   | Result of the roll                    |


## License

froll is Public Domain software available under the Creative Commons 0. See [LICENSE](LICENSE) and [COPYING](COPYING) for more information.
