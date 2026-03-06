// This file may just be the end of me, time to figure out what in the hell recursive descent is
// Top 10 things would be to not loosing my hair in the process of making this
use crate::AstNode::AstNode;
use crate::token::Token;

// pub fn parse(tokens: Vec<Token>) -> Astnode {
//     // Todo
// }

// // fn parse_expression(tokens: &[Token], current: &mut usize)
// // fn parse_term(tokens: &[Token], current: &mut usize)
// // fn parse_factor(tokens: &[Token], current: &mut usize)

// // Used for + and -, AstNode if everything correct or error string (i think, rust lowkey hard)
// fn parse_primary(tokens: &[Token], current: &mut usize) -> Result<AstNode, String> {
//     let current_token = &tokens[*current];

//     if let Token::Number(n) = current_token {
//         *current += 1;
//         return Ok(AstNode::new_number(*n));
//     }

//     Err(String::from("Expected a number"))
// }
