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
        int x = 69;
        int main() {
            int x = 67;
            return x;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    // lexer.print();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();

    let mut sym = Semantic::new();
    sym.analyze(&mut parser.ast);
    parser.print();
}
