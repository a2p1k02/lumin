use std::fs::File;
use std::io::Read;
use crate::lexer::{Lexer, TokenType};
use crate::parser::{Parser};

pub mod lexer;
pub mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let file = File::open(args[1].clone());
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
        println!("{:?}", ast);
    } else {
        println!("Usage: lumin [script]");
    }
}
