mod ast;
mod debug;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use crate::{lexer::Lexer, parser::Parser, semantic::Semantic};

fn main() {
    let source = r#"
        int main() {
            int x = "Hello";
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    // lexer.print();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();
    // parser.print();

    let mut sym = Semantic::new(&parser.ast);
    sym.analyze();
}
