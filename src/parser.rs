// This file may just be the end of me, time to figure out what in the hell recursive descent is
// Top 10 things would be to not loosing my hair in the process of making this
use crate::AstNode::AstNode;
use crate::token::Token;

pub fn parse(tokens: &[Token]) -> Result<AstNode, String> {
    // Todo
}

fn parse_expression(tokens: &[Token], current: &mut usize)
fn parse_term(tokens: &[Token], current: &mut usize){
    let left = parse_factor(tokens, current)?;

    if 
}


fn parse_factor(tokens: &[Token], current: &mut usize) -> Result<AstNode, String> {
    let left = parse_primary(tokens, current)?;

    if *current < tokens.len() && tokens[*current] == Token::Exponent {
        *current += 1;
        let right = parse_factor(tokens, current)?;
        let mut node = AstNode::new_operator(Token::Exponent);
        node.set_left(left);
        node.set_right(right);
        return Ok(node);
    }

    Ok(left)
}


// Lowest level logic is here
fn parse_primary(tokens: &[Token], current: &mut usize) -> Result<AstNode, String> {
    if *current >= tokens.len() {
        return Err(String::from("Unexpected end of input"));
    }

    match &tokens[*current] {
        Token::Number(n) => {
            let n = *n;
            *current += 1;
            Ok(AstNode::new_number(n))
        }
        Token::LeftParen => {
            *current += 1; 
            let node = parse_expression(tokens, current)?;
            if *current >= tokens.len() || tokens[*current] != Token::RightParen {
                return Err(String::from("Expected closing parenthesis"));
            }
            *current += 1;
            Ok(node)
        }
        other => Err(format!("Unexpected token: {:?}", other)),
    }
}
