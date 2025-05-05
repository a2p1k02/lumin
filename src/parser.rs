use crate::lexer::TokenType;
use crate::astnode::ASTNode;

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
    
    fn current_token(&self) -> &TokenType {
        &self.tokens[self.pos]
    }
    
    fn advance(&mut self) {
        self.pos += 1;
    }
    
    pub fn parse(&mut self) -> ASTNode {
        let mut nodes = vec![];
        while self.pos < self.tokens.len() && *self.current_token() != TokenType::EOF {
            if *self.current_token() == TokenType::FUN {
                nodes.push(self.parse_function());
            } else {
                nodes.push(self.parse_statement());
            }
        }
        ASTNode::Program(nodes)
    }
    
    fn parse_function(&mut self) -> ASTNode {
        if *self.current_token() == TokenType::FUN {
            self.advance();
        } else {
            panic!("expected 'fun'");
        }
        
        let name = match self.current_token() {
            TokenType::IDENTIFIER(s) => s.clone(),
            _ => panic!("expected function name")
        };
        
        self.advance();
        if *self.current_token() != TokenType::LPAREN {
            panic!("expected '('");
        }
        
        self.advance();
        let mut params = vec![];
        if *self.current_token() != TokenType::RPAREN {
            match self.current_token() {
                TokenType::IDENTIFIER(s) => params.push(s.clone()),
                _ => panic!("expected parameter")
            }
            self.advance();
            while *self.current_token() == TokenType::COMMA {
                self.advance();
                match self.current_token() {
                    TokenType::IDENTIFIER(s) => params.push(s.clone()),
                    _ => panic!("expected parameter")
                }
                self.advance();
            }
        }
        if *self.current_token() != TokenType::RPAREN {
            panic!("expected ')'");
        }
        
        self.advance();
        if *self.current_token() != TokenType::LBRACE {
            panic!("expected '{{'");
        }
        
        self.advance();
        
        let mut body = vec![];
        while *self.current_token() != TokenType::RBRACE {
            body.push(self.parse_statement());
        }
        self.advance();
        
        ASTNode::Function(name, params, body)
    }
    
    fn parse_statement(&mut self) -> ASTNode {
        let expr = self.parse_expression();
        if *self.current_token() == TokenType::SEMICOLON {
            self.advance();
        } else {
            panic!("expected ';'");
        }
        expr
    }
    
    fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while self.pos < self.tokens.len() {
            match self.current_token() {
                TokenType::PLUS => {
                    self.advance();
                    let right = self.parse_term();
                    node = ASTNode::BinaryOp(Box::new(node), '+', Box::new(right));
                }
                TokenType::MINUS => {
                    self.advance();
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
            match self.current_token() {
                TokenType::STAR => {
                    self.advance();
                    let right = self.parse_factor();
                    node = ASTNode::BinaryOp(Box::new(node), '*', Box::new(right));
                }
                TokenType::SLASH => {
                    self.advance();
                    let right = self.parse_factor();
                    node = ASTNode::BinaryOp(Box::new(node), '/', Box::new(right));
                }
                _ => break,
            }
        }
        
        node
    }    
    
    fn parse_factor(&mut self) -> ASTNode {
        match self.current_token() {
            TokenType::NUMBER(n) => {
                let node = ASTNode::Number(*n);
                self.advance();
                node
            }
            TokenType::IDENTIFIER(s) => {
                let name = s.clone();
                self.advance();
                if *self.current_token() == TokenType::LPAREN {
                    self.advance();
                    let mut args = vec![];
                    if *self.current_token() != TokenType::RPAREN {
                        args.push(self.parse_expression());
                        while *self.current_token() == TokenType::COMMA {
                            self.advance();
                            args.push(self.parse_expression());
                        }
                    }
                    if *self.current_token() != TokenType::RPAREN {
                        panic!("expected ')'");
                    }
                    self.advance();
                    ASTNode::Call(name, args)
                } else {
                    ASTNode::Identifier(name)
                }
            }
            TokenType::LPAREN => {
                self.advance();
                let node = self.parse_expression();
                if *self.current_token() == TokenType::RPAREN {
                    self.advance();
                    node
                } else {
                    panic!("expected ')'");
                }
            }
            _ => panic!("expected number, identifier or '('"),
        }
    }
}
