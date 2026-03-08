// This file may just be the end of me, time to figure out what in the hell recursive descent is
// Top 10 things would be to not loosing my hair in the process of making this
use crate::AstNode::AstNode;
use crate::token::Token;

pub fn parse(tokens: &[Token]) -> Result<AstNode, String> {
    // Todo
}

fn parse_expression(tokens: &[Token], current: &mut usize)
fn parse_term(tokens: &[Token], current: &mut usize)


fn parse_factor(tokens: &[Token], current: &mut usize) -> Result<AstNode, String> {
    let left = parse_primary(tokens, current)?;

    if *current < tokens.len() && tokens[*current] == Token::Caret {
        *current += 1;
        let right = parse_primary(tokens, current)?;
        let mut node = AstNode::new_operator(Token::Caret);
        node.set_left(left);
        node.set_right(right);
        return Ok(node);
    }

    Ok(left)
}


// Used for + and -, AstNode if everything correct or error string (i think, rust lowkey hard)
fn parse_primary(tokens: &[Token], current: &mut usize) -> Result<AstNode, String> {
    let current_token = &tokens[*current];

    if let Token::Number(n) = current_token {
        *current += 1;
        return Ok(AstNode::new_number(*n));
    }

    Err(String::from("Expected a number"))
}
