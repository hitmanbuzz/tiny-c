#![allow(nonstandard_style)]

mod ast;
mod debug;
mod error;
mod lexer;
mod parser;
mod token;
mod types;

use crate::{ast::Stmt, error::ErrorMsg, lexer::Lexer, parser::Parser};

fn main() {
    let source = r#"
        int main() {
            return 6.72323;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    lexer.print_err();
    lexer.print();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    parser.print();
}
