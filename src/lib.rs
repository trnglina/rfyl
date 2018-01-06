extern crate rand;
use self::rand::{thread_rng, Rng};

mod rpn;
mod infix;
mod tokens;

use tokens::match_token;
use rpn::{parse_into_rpn};
use infix::{parse_into_infix};

#[derive(Clone)]
pub struct DiceRolls {
    rolls: Vec<DiceRoll>,
    formula: Vec<String>,
    rolls_formula: Vec<String>,
}

impl DiceRolls {
    /// Returns an i32 as the result of the formula including any calculational
    /// operators.
    pub fn get_result(&self) -> i32 {
        return rpn::solve_rpn_formula(self.formula.clone());
    }

    /// Returns an i32 as the simple sum of all rolls.
    pub fn get_sum_of_rolls(&self) -> i32 {
        let mut total = 0;
        for roll in &self.rolls {
            total += roll.result;
        }
        return total;
    }

    /// Returns a formatted String showing the dice and the rolled results.
    pub fn get_rolls_string(&self) -> String {
        let mut rolls_string = String::new();
        for (i, roll) in self.rolls.iter().enumerate() {
            if i == self.rolls.len() - 1 {
                rolls_string.push_str(format!("d{} -> [{}]", roll.sides, roll.result).as_ref());
                break;
            }
            rolls_string.push_str(format!("d{} -> [{}], ", roll.sides, roll.result).as_ref());
        }
        return rolls_string;
    }

    /// Returns a postfix formatted String showing the formula.
    pub fn get_formula_string_as_rpn(&self) -> String {
        let mut formula_string = String::new();
        for (i, fragment) in self.formula.iter().enumerate() {
            if match_token(fragment) > 0 {
                formula_string.push_str(format!("{} ", fragment).as_ref());
                continue;
            }

            if i == self.formula.len() - 1 {
                formula_string.push_str(format!("[{}]", fragment).as_ref());
                break;
            }

            formula_string.push_str(format!("[{}] ", fragment).as_ref());
        }
        return formula_string;
    }

    /// Returns an infix formatted String showing the formula.
    pub fn get_formula_string_as_infix(&self) -> String {
        return parse_into_infix(self.formula.clone()).replace("( ", "[").replace(" )", "]");
    }

    /// Returns a postfix formatted String showing the formula withthe original dice notation instead of the rolled result.
    pub fn get_rolls_formula_string_as_rpn(&self) -> String {
        let mut formula_string = String::new();
        for (i, fragment) in self.rolls_formula.iter().enumerate() {
            if match_token(fragment) > 0 {
                formula_string.push_str(format!("{} ", fragment).as_ref());
                continue;
            }

            if i == self.rolls_formula.len() - 1 {
                formula_string.push_str(format!("[{}]", fragment).as_ref());
                break;
            }

            formula_string.push_str(format!("[{}] ", fragment).as_ref());
        }
        return formula_string;
    }

    /// Returns a infix formatted String showing the formula withthe original dice notation instead of the rolled result.
    pub fn get_rolls_formula_string_as_infix(&self) -> String {
        return parse_into_infix(self.rolls_formula.clone()).replace("( ", "[").replace(" )", "]");
    }
}

#[derive(Clone, Copy)]
pub struct DiceRoll {
    sides: i32,
    result: i32,
}

/// Returns a DiceRolls object based on the provided formula.
///
/// # Arguments
/// * `input` - A string that provides the dice notation to work off.
pub fn roll(input: String) -> Result<DiceRolls, Box<std::error::Error>> {
    let formula_vector = parse_into_rpn(input.trim().as_ref());
    return resolve_rolls_vector(formula_vector);
}

fn resolve_rolls_vector(rolls_vector: Vec<String>) -> Result<DiceRolls, Box<std::error::Error>> {
    let mut formula_vector: Vec<String> = Vec::new();
    let mut formula_vector_with_rolls: Vec<String> = Vec::new();
    let mut dice_rolls: Vec<DiceRoll> = Vec::new();

    for element in rolls_vector {
        // Ignore if element is recognised as a token.
        if match_token(element.as_ref()) > 0 {
            formula_vector.push(element.clone());
            formula_vector_with_rolls.push(element);
            continue;
        }

        let roll = resolve_roll_fragment(element.as_ref())?;

        for i_roll in roll.clone().rolls {
            dice_rolls.push(i_roll);
        }

        formula_vector.push(roll.get_sum_of_rolls().to_string());
        formula_vector_with_rolls.push(element);
    }

    return Ok(DiceRolls {
        rolls: dice_rolls,
        formula: formula_vector,
        rolls_formula: formula_vector_with_rolls,
    });
}

fn resolve_roll_fragment(input_fragment: &str) -> Result<DiceRolls, Box<std::error::Error>> {
    let mut rng = thread_rng();
    let mut dice_count_str = String::new();
    let mut dice_sides_str = String::new();
    let mut d_switch: bool = false;
    let mut dice_rolls: Vec<DiceRoll> = Vec::new();
    let mut sum: i32 = 0;
    let dice_count: i32;
    let dice_sides: i32;

    if input_fragment.parse::<i32>().is_ok() {
        let current_roll = DiceRoll {
            sides: 0,
            result: input_fragment.parse::<i32>().unwrap(),
        };

        dice_rolls.push(current_roll);
        sum += current_roll.result;
    } else {
        for (i, c) in input_fragment.chars().enumerate() {
            if !d_switch {
                if c.to_string() == "d" {
                    d_switch = true;
                    if i == 0 {
                        dice_count_str.push_str("1");
                    }
                    continue;
                }
                dice_count_str.push(c);
            } else {
                dice_sides_str.push(c);
            }
        }

        dice_count = dice_count_str.parse::<i32>()?;
        let dice_sides_result = dice_sides_str.parse::<i32>();
        if dice_sides_result.is_ok() {
            dice_sides = dice_sides_result.unwrap();            
        } else if match_token(dice_sides_str.as_ref()) == -3 {
            dice_sides = 100;
        } else {
            return Err(Box::new(dice_sides_result.unwrap_err()));
        }
                
        for _ in 0..dice_count {
            let current_roll = DiceRoll {
                sides: dice_sides,
                result: rng.gen_range(1, dice_sides),
            };

            dice_rolls.push(current_roll);
            sum += current_roll.result;
        }
    }

    return Ok(DiceRolls {
        rolls: dice_rolls,
        formula: vec![sum.to_string()],
        rolls_formula: vec![input_fragment.to_string()],
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_from_string() {
        println!();
        let roll0 = roll("2d4".to_string()).unwrap();
        println!("Rolls:             {}", roll0.get_rolls_string());
        println!("RPN Formula:       {}", roll0.get_formula_string_as_rpn());
        println!("Formula:           {}", roll0.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll0.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll0.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll0.get_result());
        println!();

        let roll1 = roll("(2d6 - 1d8) * (3d4 + 4d12)".to_string()).unwrap();
        println!("Rolls:             {}", roll1.get_rolls_string());
        println!("RPN Formula:       {}", roll1.get_formula_string_as_rpn());
        println!("Formula:           {}", roll1.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll1.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll1.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll1.get_result());
        println!();

        let roll2 = roll("3d% + d%".to_string()).unwrap();
        println!("Rolls:             {}", roll2.get_rolls_string());
        println!("RPN Formula:       {}", roll2.get_formula_string_as_rpn());
        println!("Formula:           {}", roll2.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll2.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll2.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll2.get_result());
        println!();

        let roll3 = roll("d100 / 15".to_string()).unwrap();
        println!("Rolls:             {}", roll3.get_rolls_string());
        println!("RPN Formula:       {}", roll3.get_formula_string_as_rpn());
        println!("Formula:           {}", roll3.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll3.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll3.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll3.get_result());
        println!();

        let roll4 = roll("1d4 + 2d6 * 3d2 / 4d8 + (2d6 + 3d8) - 16 * (1 / 1d4)".to_string()).unwrap();
        println!("Rolls:             {}", roll4.get_rolls_string());
        println!("RPN Formula:       {}", roll4.get_formula_string_as_rpn());
        println!("Formula:           {}", roll4.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll4.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll4.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll4.get_result());
        println!();
    }
}
