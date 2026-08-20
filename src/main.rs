mod ast;
mod debug;
mod error;
mod lexer;
mod parser;
mod token;
mod types;

use crate::{error::ErrorMsg, lexer::Lexer, parser::Parser};

fn main() {
    let source = r#"
        int x = 6;
        int y = x;
        int main() {
            int a = 5;
            int hello = a;
            return 69;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    if !lexer.errors.is_empty() {
        lexer.print_err();
    } else {
        lexer.print();
    }

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    parser.print();
}
