mod lexer;
mod parser;

use lexer::{lexer::Lexer, token::Token};
use parser::{ast::Expression, parser::Parser};

fn main() {
    let program = "-10 + -2/3";
    let mut lexer = Lexer::new(program);

    let tokens: Vec<Token> = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err.msg);
            std::process::exit(1);
        },
    };

    let mut parser = Parser::new(tokens);
    let ast: Expression = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("{}", err.msg);
            std::process::exit(1);
        }
    };

    println!("{:?}", ast);
}
