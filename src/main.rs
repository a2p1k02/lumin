use crate::lexer::{Lexer, TokenType};
use crate::parser::{Parser};

pub mod lexer;
pub mod parser;

fn main() {
    let input = String::from("2 + 3 * 4");
    let mut lexer = Lexer::from(input);
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
}
