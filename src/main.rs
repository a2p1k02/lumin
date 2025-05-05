use crate::lexer::{Lexer, TokenType};

pub mod lexer;

fn main() {
    let input = String::from("2 + 2 * 2");
    let mut lexer = Lexer::from(input);
    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == TokenType::EOF {
            break;
        }
    }
}
