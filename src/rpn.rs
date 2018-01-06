//! Provides facilities for parsing and solving reverse Polish notation dice specifications.
use tokens::match_token;

/// Returns a Vector of Strings with each element containing a token or an operator in postfix (rpn) format.
///
/// # Arguments
/// * `input_formula` - A string that provides the notation to work off.
///
/// # Example values
///
/// * `3 + 4 * 6` -> `["3", "4", "6", "*", "+"]`
/// * `2d4 + d6 + d4` -> `["2d4", "d6", "d4", "+", "+"]`
/// * `xv * (ab + dc)` -> `["xv", "ab", "dc", "+", "*"]`
pub fn parse_into_rpn(input_formula: &str) -> Vec<String> {
    let formula = input_formula.replace(" ", "").replace("_", "");
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
fn parse_rpn_formula() {
    assert_eq!(vec!["3", "4", "+"], parse_into_rpn("3 + 4"));
    assert_eq!(
        vec!["3", "4", "2", "1", "−", "×", "+"],
        parse_into_rpn("3 + 4 × (2 − 1)")
    );
    assert_eq!(
        vec!["2", "1", "−", "3", "×", "4", "+"],
        parse_into_rpn("(2 − 1) × 3 + 4")
    );
    assert_eq!(vec!["x", "y", "+"], parse_into_rpn("x + y"));
    assert_eq!(
        vec!["ab", "cd", "ef", "gh", "−", "×", "+"],
        parse_into_rpn("ab + cd × (ef − gh)")
    );
    assert_eq!(
        vec!["2d5", "1d6", "−", "3d6", "×", "2d12", "+"],
        parse_into_rpn("(2d5 − 1d6) × 3d6 + 2d12")
    );
}

/// Returns an i32 as the result of a postfix (rpn) formula.
///
/// # Arguments
/// * `formula` - A Vector of Strings that provides the postfix formatted notation to work off.
/// See [rfyl::parse_into_rpn()](fn.parse_into_rpn.html) for more details.
///
/// # Example values
///
/// * `["3", "4", "6", "*", "+"]` -> `27`
pub fn solve_rpn_formula(formula: Vec<String>) -> i32 {
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
                            if a == 0 {panic!("Divide by zero: `{} / {}` is undefined", b, a);}
                            working_stack.push((b as f32 / a as f32).round() as i32)
                        },
                        3 => working_stack.push(b * a),
                        2 => working_stack.push(b + a),
                        1 => working_stack.push(b - a),
                        _ => panic!("Invalid operator: `{}`", e),
                    }
                } else {
                    panic!("Right hand token in evaluation doesn't exist");
                }
            } else {
                panic!("Left hand token in evaluation doesn't exist");
            }
        }
    }
    if let Some(t) = working_stack.pop() {
        total = t;
    }
    return total;
}

#[test]
fn solve_rpn() {
    assert_eq!(
        6,
        solve_rpn_formula(vec![
            "4".to_string(),
            "2".to_string(),
            "+".to_string(),
        ])
    );
    assert_eq!(
        5,
        solve_rpn_formula(vec![
            "2".to_string(),
            "2".to_string(),
            "*".to_string(),
            "4".to_string(),
            "4".to_string(),
            "*".to_string(),
            "+".to_string(),
            "4".to_string(),
            "/".to_string(),
        ])
    );
}
