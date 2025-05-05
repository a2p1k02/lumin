use crate::lexer::TokenType;

#[derive(Debug)]
pub enum ASTNode {
    Number(f64),
    BinaryOp(Box<ASTNode>, char, Box<ASTNode>),
}

pub struct Parser {
    tokens: Vec<TokenType>,
    pos: usize
}

impl Parser {
    pub fn from(tokens: Vec<TokenType>) -> Self {
        Self {
            tokens, 
            pos: 0
        }
    }
    
    pub fn parse(&mut self) -> ASTNode {
        let node = self.parse_expression();
        if self.pos < self.tokens.len() && self.tokens[self.pos] != TokenType::EOF {
            panic!("unexpected tokens");
        }
        node
    }
    
    fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                TokenType::PLUS => {
                    self.pos += 1;
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), '+', Box::new(right));
                }
                TokenType::MINUS => {
                    self.pos += 1;
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), '-', Box::new(right));
                }
                _ => break,
            }
        }
        
        node
    }    
    
    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while self.pos < self.tokens.len() {
            match self.tokens[self.pos] {
                TokenType::STAR => {
                    self.pos += 1;
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), '*', Box::new(right));
                }
                TokenType::SLASH => {
                    self.pos += 1;
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), '/', Box::new(right));
                }
                _ => break,
            }
        }
        
        node
    }    
    
    fn parse_factor(&mut self) -> ASTNode {
        let token = &self.tokens[self.pos];
        match token {
            TokenType::NUMBER(n) => {
                self.pos += 1;
                ASTNode::Number(*n)
            }
            _ => panic!("expected number but got: {:?}", token),
        }
    }
}
