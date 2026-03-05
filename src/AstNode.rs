use crate::token::Token;


// Struct definition, 
pub struct AstNode{
    left_child: Option<Box<AstNode>>,
    right_child: Option<Box<AstNode>>,
    operator: Option<Token>,
    number: Option<f64>,
}


impl AstNode{

    pub fn new(operator: Option<Token>, number: Option<f64>) -> AstNode{
        AstNode{
            operator, 
            number,
            left_child: None,
            right_child: None,
        }
    }
    

    
}