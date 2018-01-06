//! Provides facilities for parsing a vector into infix.
use tokens::match_token;

/// Returns a Vector of Strings with each element containing a token or an operator in bracketed infix format.
///
/// # Arguments
/// * `input_formula` - A Vector of Strings that provides the postfix formatted notation to work off.
/// See [rfyl::parse_into_rpn()](fn.parse_into_rpn.html) for more details.
///
/// # Example values
///
/// * `["3", "4", "6", "*", "+"]` -> `["(", "3", "+", "(", "4", "*", "6", ")", ")"]`
pub fn parse_into_infix(input_formula: Vec<String>) -> String {
    let mut formula_vector: Vec<String> = Vec::new();
    let mut formula_string = String::new();

    for e in input_formula {
        let precedence = match_token(e.as_ref());

        match precedence {
            // Operator
            p if p > 0 => if formula_vector.len() < 2 {
                panic!("Insufficient values in expression start");
            } else {
                if let Some(a) = formula_vector.pop() {
                    if let Some(b) = formula_vector.pop() {
                        formula_vector.push(format!("( {0} {1} {2} )", b, e, a));
                    } else {
                        panic!("Right hand token in evaluation doesn't exist");
                    }
                } else {
                    panic!("Left hand token in evaluation doesn't exist");
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
        panic!("Too many values in postfix formula. Please verify the formula.");
    } else if formula_vector.len() < 1 {
        panic!("Not enough values in postfix formula. Please verify the formula.");
    }

    return formula_string;
}

#[test]
fn parse_infix_formula() {
    assert_eq!(
        "( 3 + 4 )",
        parse_into_infix(vec!["3".to_string(), "4".to_string(), "+".to_string()])
    );
    assert_eq!(
        "( 3 + ( 4 × ( 2 − 1 ) ) )",
        parse_into_infix(vec![
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
        parse_into_infix(vec![
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