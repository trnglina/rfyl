extern crate rand;

pub mod rfyl {
    use rand::{thread_rng, Rng};

    #[derive(Clone)]
    pub struct DiceRolls {
        rolls: Vec<DiceRoll>,
        formula: Vec<String>,
        rolls_formula: Vec<String>,
    }

    impl DiceRolls {
        pub fn get_result(&self) -> i32 {
            return solve_rpn_formula(self.formula.clone());
        }

        pub fn get_sum_of_rolls(&self) -> i32 {
            let mut total = 0;
            for roll in &self.rolls {
                total += roll.result;
            }
            return total;
        }

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

        pub fn get_formula_string_as_infix(&self) -> String {
            return parse_formula_into_infix(self.formula.clone());
        }

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

        pub fn get_rolls_formula_string_as_infix(&self) -> String {
            return parse_formula_into_infix(self.rolls_formula.clone());
        }
    }

    #[derive(Clone, Copy)]
    pub struct DiceRoll {
        sides: i32,
        result: i32,
    }

    pub fn roll(input: &str) -> DiceRolls {
        let formula_vector = parse_formula_into_rpn(input);
        return resolve_rolls_vector(formula_vector);
    }

    fn parse_formula_into_rpn(input_formula: &str) -> Vec<String> {
        let formula = input_formula.replace(" ", "");
        let mut formula_vector: Vec<String> = Vec::new();
        let mut active_segment = String::new();
        let mut operator_stack: Vec<String> = Vec::new();
        let mut lorb = false;

        for c in formula.chars() {
            let cs = c.to_string();
            let precedence = match_token(cs.as_ref());

            match precedence {
                // Current token is an operator token
                p if p > 0 => if active_segment.len() > 0 {
                    formula_vector.push(active_segment.clone());
                    active_segment = String::new();
                    while let Some(top) = operator_stack.pop() {
                        if match_token(top.as_ref()) >= precedence {
                            formula_vector.push(top.to_string());
                        } else {
                            operator_stack.push(top);
                            break;
                        }
                    }
                    operator_stack.push(cs);
                } else if lorb {
                    operator_stack.push(cs);
                } else {
                    active_segment.push(c);
                },
                // Current token is a left bracket token
                p if p == -1 => {
                    lorb = false;
                    operator_stack.push(cs);
                }
                // Current token is a right bracket token
                p if p == -2 => {
                    if active_segment.len() > 0 {
                        formula_vector.push(active_segment.clone());
                        active_segment = String::new();
                        lorb = true;
                    }
                    while let Some(top) = operator_stack.pop() {
                        if match_token(top.as_ref()) == -1 {
                            break;
                        }
                        formula_vector.push(top.to_string());
                    }
                }
                // Current token is a standard token
                _ => {
                    lorb = false;
                    active_segment.push(c);
                }
            }
        }

        if active_segment.len() > 0 {
            formula_vector.push(active_segment);
        }

        while let Some(top) = operator_stack.pop() {
            formula_vector.push(top.to_string());
        }

        return formula_vector;
    }

    #[test]
    fn parse_into_rpn() {
        assert_eq!(vec!["3", "4", "+"], parse_formula_into_rpn("3 + 4"));
        assert_eq!(
            vec!["3", "4", "2", "1", "−", "×", "+"],
            parse_formula_into_rpn("3 + 4 × (2 − 1)")
        );
        assert_eq!(
            vec!["2", "1", "−", "3", "×", "4", "+"],
            parse_formula_into_rpn("(2 − 1) × 3 + 4")
        );
        assert_eq!(vec!["x", "y", "+"], parse_formula_into_rpn("x + y"));
        assert_eq!(
            vec!["ab", "cd", "ef", "gh", "−", "×", "+"],
            parse_formula_into_rpn("ab + cd × (ef − gh)")
        );
        assert_eq!(
            vec!["2d5", "1d6", "−", "3d6", "×", "2d12", "+"],
            parse_formula_into_rpn("(2d5 − 1d6) × 3d6 + 2d12")
        );
    }

    fn parse_formula_into_infix(input_formula: Vec<String>) -> String {
        let mut formula_vector: Vec<String> = Vec::new();
        let mut formula_string = String::new();

        for e in input_formula {
            let precedence = match_token(e.as_ref());

            match precedence {
                // Operator
                p if p > 0 => if formula_vector.len() < 2 {
                    panic!("Insufficient values in expression start!");
                } else {
                    if let Some(a) = formula_vector.pop() {
                        if let Some(b) = formula_vector.pop() {
                            formula_vector.push(format!("( {0} {1} {2} )", b, e, a));
                        } else {
                            panic!("Right hand token in evaluation doesn't exist!");
                        }
                    } else {
                        panic!("Left hand token in evaluation doesn't exist!");
                    }
                },
                // Non-operator
                _ => {
                    formula_vector.push(e);
                }
            }
        }


        if formula_vector.len() == 1 {
            formula_string = formula_vector[0].to_string();
        } else if formula_vector.len() > 1 {
            panic!("Too many values!");
        } else if formula_vector.len() < 1 {
            panic!("Not enough values!");
        }

        return formula_string;
    }

    #[test]
    fn parse_into_infix() {
        assert_eq!(
            "( 3 + 4 )",
            parse_formula_into_infix(vec!["3".to_string(), "4".to_string(), "+".to_string()])
        );
        assert_eq!(
            "( 3 + ( 4 × ( 2 − 1 ) ) )",
            parse_formula_into_infix(vec![
                "3".to_string(),
                "4".to_string(),
                "2".to_string(),
                "1".to_string(),
                "−".to_string(),
                "×".to_string(),
                "+".to_string(),
            ])
        );
        assert_eq!(
            "( ( ( 2 − 1 ) × 3 ) + 4 )",
            parse_formula_into_infix(vec![
                "2".to_string(),
                "1".to_string(),
                "−".to_string(),
                "3".to_string(),
                "×".to_string(),
                "4".to_string(),
                "+".to_string(),
            ])
        );
    }

    fn resolve_rolls_vector(rolls_vector: Vec<String>) -> DiceRolls {
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

            let roll = resolve_roll_fragment(element.as_ref());

            for i_roll in roll.clone().rolls {
                dice_rolls.push(i_roll);
            }

            formula_vector.push(roll.get_sum_of_rolls().to_string());
            formula_vector_with_rolls.push(element);
        }

        return DiceRolls {
            rolls: dice_rolls,
            formula: formula_vector,
            rolls_formula: formula_vector_with_rolls,
        };
    }

    fn resolve_roll_fragment(input_fragment: &str) -> DiceRolls {
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

            dice_count = dice_count_str.parse::<i32>().unwrap();
            dice_sides = dice_sides_str.parse::<i32>().unwrap();
            
            for _ in 0..dice_count {
                let current_roll = DiceRoll {
                    sides: dice_sides,
                    result: rng.gen_range(1, dice_sides),
                };

                dice_rolls.push(current_roll);
                sum += current_roll.result;
            }
        }

        return DiceRolls {
            rolls: dice_rolls,
            formula: vec![sum.to_string()],
            rolls_formula: vec![input_fragment.to_string()],
        };
    }

    fn solve_rpn_formula(formula: Vec<String>) -> i32 {
        let mut working_stack: Vec<i32> = Vec::new();
        let mut total: i32 = 0;
        for e in formula.iter() {
            if e.parse::<i32>().is_ok() {
                working_stack.push(e.parse::<i32>().unwrap());
            } else {
                if let Some(a) = working_stack.pop() {
                    if let Some(b) = working_stack.pop() {
                        match match_token(e) {
                            4 => {
                                if a == 0 {panic!("Divide by zero!");}
                                working_stack.push((b as f32 / a as f32).round() as i32)
                            },
                            3 => working_stack.push(b * a),
                            2 => working_stack.push(b + a),
                            1 => working_stack.push(b - a),
                            _ => panic!("Invalid operator!"),
                        }
                    } else {
                        panic!("Right hand token in evaluation doesn't exist!");
                    }
                } else {
                    panic!("Left hand token in evaluation doesn't exist!");
                }
            }
        }
        if let Some(t) = working_stack.pop() {
            total = t;
        }
        return total;
    }

    fn match_token(token: &str) -> i32 {
        match token {
            "/" => return 4,
            "÷" => return 4,
            "*" => return 3,
            "×" => return 3,
            "+" => return 2,
            "−" => return 1,
            "-" => return 1,
            "(" => return -1,
            ")" => return -2,
            _ => return 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use rfyl;

    #[test]
    fn roll_from_string() {
        println!();
        let roll1 = rfyl::roll("2d6 - 1d8 * 3d4 + 4d12");
        println!("Rolls:             {}", roll1.get_rolls_string());
        println!("RPN Formula:       {}", roll1.get_formula_string_as_rpn());
        println!("Formula:           {}", roll1.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll1.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll1.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll1.get_result());
        println!();

        let roll2 = rfyl::roll("d12 - d8 + d5 * d18");
        println!("Rolls:             {}", roll2.get_rolls_string());
        println!("RPN Formula:       {}", roll2.get_formula_string_as_rpn());
        println!("Formula:           {}", roll2.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll2.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll2.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll2.get_result());
        println!();

        let roll3 = rfyl::roll("d100 / 15");
        println!("Rolls:             {}", roll3.get_rolls_string());
        println!("RPN Formula:       {}", roll3.get_formula_string_as_rpn());
        println!("Formula:           {}", roll3.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll3.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll3.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll3.get_result());
        println!();

        let roll4 = rfyl::roll("1d4 + 2d6 * 3d2 / 4d8 + (2d6 + 3d8) - 16 * (1 / 1d4)");
        println!("Rolls:             {}", roll4.get_rolls_string());
        println!("RPN Formula:       {}", roll4.get_formula_string_as_rpn());
        println!("Formula:           {}", roll4.get_formula_string_as_infix());
        println!("RPN Rolls Formula: {}", roll4.get_rolls_formula_string_as_rpn());
        println!("Rolls Formula:     {}", roll4.get_rolls_formula_string_as_infix());
        println!("Result:            {}", roll4.get_result());
        println!();
    }
}
