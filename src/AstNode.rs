use crate::token::Token;

// Struct definition,
pub struct AstNode {
    left_child: Option<Box<AstNode>>,
    right_child: Option<Box<AstNode>>,
    operator: Option<Token>,
    number: Option<f64>,
}

impl AstNode {
    // Base associated function, can later set stuff like child nodes and operations
    pub fn new(operator: Option<Token>, number: Option<f64>) -> AstNode {
        AstNode {
            operator,
            number,
            left_child: None,
            right_child: None,
        }
    }

    // Associated function for only the number nodes
    pub fn new_number(number: f64) -> AstNode {
        AstNode {
            operator: None,
            number: Some(number),
            left_child: None,
            right_child: None,
        }
    }

    // Associated function for only the operator nodes
    pub fn new_operator(operator: Token) -> AstNode {
        AstNode {
            operator: Some(operator),
            number: None,
            left_child: None,
            right_child: None,
        }
    }

    // Getter for number
    pub fn get_num(&self) -> Option<f64> {
        self.number
    }

    // Getter for operator
    pub fn get_operator(&self) -> Option<&Token> {
        self.operator.as_ref()
    }

    // Setter for number
    pub fn set_num(&mut self, number: f64) {
        self.number = Some(number);
    }

    // Setter for operator
    pub fn set_operator(&mut self, operator: Token) {
        self.operator = Some(operator);
    }

    // Getter for left node
    pub fn get_left(&self) -> Option<&AstNode> {
        self.left_child.as_deref()
    }

    // Getter for right node
    pub fn get_right(&self) -> Option<&AstNode> {
        self.right_child.as_deref()
    }

    // Setter for left node
    pub fn set_left(&mut self, node: AstNode) {
        self.left_child = Some(Box::new(node));
    }

    // Setter for right node
    pub fn set_right(&mut self, node: AstNode) {
        self.right_child = Some(Box::new(node));
    }
}
