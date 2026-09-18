mod ast;
mod debug;
mod error;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use crate::{error::ErrorMsg, lexer::Lexer, parser::Parser, semantic::Symantic};

fn main() {
    let source = r#"
        int main() {
            return;
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

    let sym = Symantic::new(&parser.ast);
    sym.analyze();
}
