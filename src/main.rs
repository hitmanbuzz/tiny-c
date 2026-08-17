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
            return 69;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    lexer.print_err();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    parser.print();
}
