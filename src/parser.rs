use crate::token::{
    Token,
    TokenType
};

use crate::ast::{
    BinaryOp,
    BinaryExpr, AstNode
};

pub struct Parser{
    tokens: Vec<Token>,
    index: usize,
    nodes: Vec<Box<dyn AstNode>>
}

impl Parser {

    pub fn not_implemented_yet(&self,message: &str){
	println!("Not Implemented Yet: {}", message);
    }

    pub fn new(tokens: Vec<Token>) -> Self{
	return Self{tokens, index: 0, nodes: vec!()} 
    }

    pub fn next_token(&self) -> Token {
	let next_token = self.tokens.get(self.index + 1).unwrap();
	return next_token.to_owned();
    }

    pub fn current_token(&mut self) -> Token {
	let current_token = self.tokens.get(self.index).unwrap();
	self.index = self.index+1;
	return current_token.to_owned();
    }

    pub fn peek(self) -> Token {
	return self.tokens.get(self.index).unwrap().to_owned();
    }

    pub fn parse(&mut self) {
	while self.index < self.tokens.len(){
	    let token = self.current_token();
	    match token.token_type{
		TokenType::Plus => {
		    self.nodes.push(Box::new(BinaryExpr{
			binary_op: BinaryOp::PLUS,
			next: self.next_token(),
			previous: self.previous_token()
		    }));},
		TokenType::Minus => {
		    self.nodes.push(Box::new(BinaryExpr{
			binary_op: BinaryOp::MINUS,
			next: self.next_token(),
			previous: self.previous_token()
		    }));
		},
		TokenType::Product => {
		    self.nodes.push(Box::new(BinaryExpr{
			binary_op: BinaryOp::MULTIPLY,
			next: self.next_token(),
			previous: self.previous_token()
		    }));
		},
		TokenType::Divide => {
		    self.nodes.push(Box::new(BinaryExpr{
			binary_op: BinaryOp::DIVIDE,
			next: self.next_token(),
			previous: self.previous_token()
		    }));
		},
		TokenType::Equals => {self.not_implemented_yet("Equals");},
		TokenType::Compare => {self.not_implemented_yet("Compare");},
		TokenType::NotEquals => {self.not_implemented_yet("NotEquals");},
		TokenType::Comment => {self.not_implemented_yet("Comment");},
		TokenType::Literal => {self.not_implemented_yet("Literal");},
		TokenType::Number => {self.not_implemented_yet("Number");},
		TokenType::Space => {self.not_implemented_yet("Space");},
		TokenType::Newline => {self.not_implemented_yet("NewLine");},
	    }
	}
    }

    pub fn previous_token(&self) -> Token {
	println!("INDEX: {}", self.index);
	if self.index == 0 {
	   panic!("Already at the beginning"); 
	}
	let previous_token = self.tokens.get(self.index - 1).unwrap();
	if previous_token.token_type == TokenType::Space {
	   return self.previous_token();
	}
	return previous_token.to_owned();
    }

    pub fn print_nodes(self){
        println!("=====================================");
        println!("Printing Tokens");
        for node in &self.nodes{
            println!("Node: {:?}", node.print());
        }
        println!("=====================================");
    }
}
