mod ast;
mod debug;
mod error;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use crate::{error::ErrorMsg, lexer::Lexer, parser::Parser, semantic::Semantic};

fn main() {
    let source = r#"
        void test;
        int main() {
            int x = 1 + test;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    if !lexer.errors.is_empty() {
        lexer.print_err();
        return;
    } else {
        lexer.print();
    }

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    parser.print();

    let mut sym = Semantic::new(&parser.ast);
    sym.analyze();
}
