mod constant;
mod lexer;
mod parser;
mod token;
mod ast;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;

fn usage() {
    println!("Usage: pyRust <filename>");
}

fn read_file(filepath: &str) -> String {
    return fs::read_to_string(filepath).unwrap();
}

fn execute(filename: &str) {
    let file_content = read_file(filename);
    println!("File Content: {}", file_content);
    let mut lex = Lexer::new(&file_content);
    lex.tokenize();
    lex.print_tokens();

    let mut parser = Parser::new(lex.get_tokens());
    parser.parse();
    parser.print_nodes();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
        return ();
    }
    let filepath = args.get(1).unwrap();
    return execute(filepath);
}
