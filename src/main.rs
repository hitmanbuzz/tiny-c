mod ast;
mod debug;
mod ir;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use std::fs;

use crate::{ir::IrGen, lexer::Lexer, parser::Parser, semantic::Semantic};

fn main() {
    let source = include_str!("../tests/source.tc");

    let mut lexer = Lexer::new(source);
    lexer.tokenize();
    // lexer.print();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();

    let mut sym = Semantic::new();
    sym.analyze(&mut parser.ast);

    // parser.print();

    // it will generate LLVM IR code
    let mut ir = IrGen::new(&parser.ast);
    ir.gen_ir();
    let ir_source = ir.get_ir();

    fs::write("tests/output.ll", ir_source).unwrap();
}
