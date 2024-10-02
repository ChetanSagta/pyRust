use crate::{
    constant::EOL,
    token::{Token, TokenType},
};

pub struct Lexer<'a> {
    row: i32,
    col: i32,
    index: usize,
    content: String,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input_string: &str) -> Self {
        Self {
            row: 0,
            col: 0,
            index: 0,
            content: String::from(input_string),
            tokens: Vec::new(),
        }
    }
    pub fn tokenize(&mut self) {
        let ch: char = self.next_token();
        match ch {
            '+' => self.tokens.push(Token {
                token_type: TokenType::Plus,
                value: "+",
            }),
            ' ' => self.tokens.push(Token {
                token_type: TokenType::Plus,
                value: " ",
            }),
            '\n' => self.tokens.push(Token {
                token_type: TokenType::Plus,
                value: EOL,
            }),
            _ => {
                if self.is_digit() {
                    self.numbers();
                } else if self.is_alphabet() {
                    self.alphanumberic();
                } else {
                    panic!("Unsupported Character: {}", self.next_token());
                }
            }
        }
    }

    pub fn next_token(&mut self) -> char {
        if self.index >= self.content.len() {
            panic!("Trying to next_token outside the file content");
        }
        let content = self.peek();
        self.index = self.index + 1;
        return content;
    }

    pub fn peek(&self) -> char {
        if self.index >= self.content.len() {
            panic!("Trying to next_token outside the file content");
        }
        let content = self.content.chars().nth(self.index).unwrap();
        println!("Content : {}", content);
        return content;
    }

    pub fn eol(&mut self) -> bool {
        self.next_token() == '\n'
    }

    pub fn numbers(&mut self) {
        let mut numbers: String = "".to_string();
        numbers.push(self.next_token());
        while self.is_digit() {
            numbers.push(self.next_token());
        }
    }

    pub fn alphanumberic(&mut self) {
        let mut numbers: String = "".to_string();
        numbers.push(self.next_token());
        while self.is_alpha_numeric() || self.is_digit() {
            numbers.push(self.next_token());
        }
    }

    pub fn is_digit(&mut self) -> bool {
        if self.next_token() >= '0' && self.next_token() <= '9' {
            return true;
        }
        return false;
    }

    pub fn is_alphabet(&mut self) -> bool {
        if (self.next_token() >= 'a' && self.next_token() <= 'z')
            || (self.next_token() >= 'A' && self.next_token() <= 'Z')
        {
            return true;
        }
        return false;
    }

    pub fn is_alpha_numeric(&mut self) -> bool {
        return self.is_alphabet() || self.is_digit();
    }

    pub fn print_tokens(self) {
        for token in self.tokens {
            println!("Token: {:?}", token);
        }
    }
}
