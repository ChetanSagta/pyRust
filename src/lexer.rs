use crate::{
    constant::EOF,
    constant::EOL,
    token::{Token, TokenType},
};

pub struct Lexer {
    row: i32,
    col: i32,
    index: usize,
    content: String,
    tokens: Vec<Token>,
}

impl Lexer {
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
        while self.peek() != EOF {
            let ch: char = self.next_token();
            match ch {
                '+' => self.tokens.push(Token {
                    token_type: TokenType::Plus,
                    value: "+".to_string(),
                }),
                ' ' => self.tokens.push(Token {
                    token_type: TokenType::Space,
                    value: " ".to_string(),
                }),
                '\n' => self.tokens.push(Token {
                    token_type: TokenType::Newline,
                    value: String::from(EOL),
                }),
		EOF => {
		    println!("Lexing Ended");
		    return;
		}
                _ => {
                    if self.is_digit() {
                        let numbers = self.numbers();
                        let token = Token {
                            token_type: TokenType::Number,
                            value: numbers,
                        };
                        self.tokens.push(token.clone());
                    } else if self.is_alphabet() {
                        self.alphanumberic();
                    } else {
                        panic!("Unsupported Character: {}", self.next_token());
                    }
                }
            }
        }
    }

    pub fn next_token(&mut self) -> char {
        if self.index >= self.content.len() {
            return EOF;
        }
        let content = self.peek();
        self.index = self.index + 1;
        return content;
    }

    pub fn peek(&self) -> char {
        if self.index >= self.content.len() {
	    return EOL;
        }
        let content = self.content.chars().nth(self.index).unwrap();
        return content;
    }

    pub fn eol(&mut self) -> bool {
        self.peek() == EOL
    }

    pub fn numbers(&mut self) -> String {
        let mut numbers: String = String::new();
        numbers.push(self.peek());
        while self.is_digit() {
            numbers.push(self.next_token());
        }
        return numbers;
    }

    pub fn alphanumberic(&mut self) {
        let mut alphanumeric: String = "".to_string();
        println!("AlphaNumeric Start");
        alphanumeric.push(self.peek());
        while self.is_alpha_numeric() || self.is_digit() {
            alphanumeric.push(self.next_token());
        }
        println!("AlphaNumeric End");
    }

    pub fn is_digit(&mut self) -> bool {
        let ch = self.peek();
        if ch >= '0' && ch <= '9' {
            println!("Digit");
            return true;
        }
        println!("Not Digit");
        return false;
    }

    pub fn is_alphabet(&mut self) -> bool {
        let ch = self.peek();
        if (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') {
            println!("Alphabet");
            return true;
        }
        println!("Not Alphabet");
        return false;
    }

    pub fn is_alpha_numeric(&mut self) -> bool {
        return self.is_alphabet() || self.is_digit();
    }

    pub fn print_tokens(self) {
        println!("=====================================");
        println!("Printing Tokens");
        for token in self.tokens {
            println!("Token: {:?}", token);
        }
        println!("=====================================");
    }
}
