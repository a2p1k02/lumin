use std::fs::File;
use std::io::Read;
use crate::lexer::{Lexer, TokenType};
use crate::parser::Parser;
use crate::context::Context;

pub struct Compiler {
    arg: String
}

impl Compiler {
    pub fn from(arg: String) -> Self {
        Self {
            arg
        }
    }
    
    pub fn run(&self) {
        let file = File::open(self.arg.clone());
        let mut content = String::new();
        file.unwrap().read_to_string(&mut content).unwrap();

        let mut lexer = Lexer::from(content);
        let mut tokens = vec![];
        loop {
            let token = lexer.next_token();
            tokens.push(token.clone());
            if token == TokenType::EOF {
                break;
            }
        }
        
        let mut parser = Parser::from(tokens);
        let ast = parser.parse();
        let mut context = Context::new();
        let result = context.evaluate(&ast);
        println!("{result}");
    }
}
