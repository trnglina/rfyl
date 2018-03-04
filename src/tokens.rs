pub fn match_token(token: &str) -> i32 {
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
        "%" => return -3,
        _ => return 0,
    }
}