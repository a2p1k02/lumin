#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOF,
    EQUAL,
    NUMBER(f64),
    PLUS,
    MINUS,
    STAR,
    SLASH,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize
}

impl Lexer {
    pub fn from(src: String) -> Self {
        Lexer {
            input: src.chars().collect(),
            pos: 0
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return TokenType::EOF;
        }
        let ch = self.input[self.pos];
        self.pos += 1;

        match ch {
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => TokenType::STAR,
            '/' => TokenType::SLASH,
            '0'..='9' => {
                let mut num_str = String::new();
                num_str.push(ch);
                
                while self.pos < self.input.len() && (self.input[self.pos].is_digit(10) || self.input[self.pos] == '.') {
                    num_str.push(self.input[self.pos]);
                    self.pos += 1;
                }
                TokenType::NUMBER(num_str.parse().unwrap_or(0.0))
            }
            _ => panic!("unexpected syntax: {}", ch),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}
