// This one should be easier

pub fn evaluate(node: &AstNode) -> Result<f64, String> {
    if let Some(n) = node.get_num() {
        return Ok(n);
    }

    let left = evaluate(node.get_left().ok_or("Missing left child")?)?;
    let right = evaluate(node.get_right().ok_or("Missing right child")?)?;

    match node.get_operator() {
        Some(Token::Plus)     => Ok(left + right),
        Some(Token::Minus)    => Ok(left - right),
        Some(Token::Multiply) => Ok(left * right),
        Some(Token::Divide)   => {
            if right == 0.0 {
                Err(String::from("Division by zero"))
            } else {
                Ok(left / right)
            }
        }
        Some(Token::Exponent) => Ok(left.powf(right)),
        _ => Err(String::from("Unknown operator")),
    }
}
