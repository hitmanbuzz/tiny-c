mod ast;
mod debug;
mod ir;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use std::{env, fs};

use crate::{ir::IrGen, lexer::Lexer, parser::Parser, semantic::Semantic};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("bad args: <program> <file-path>");
        return;
    }

    let file_path = &args[1];

    // didn't know you don't need `mut`
    // i guess it is not initialized and doesn't need mutate since there is no any data on it
    let source: String;

    if let Ok(content) = fs::read_to_string(file_path) {
        source = content;
    } else {
        // fuck the error msg
        eprintln!("failed to read file: {}", file_path);
        return;
    }

    let mut lexer = Lexer::new(&source);
    lexer.tokenize();
    // lexer.print();

    let mut parser = Parser::new(lexer.tokens);
    parser.parse();

    let mut sym = Semantic::new();
    let has_err = sym.analyze(&mut parser.ast);
    if has_err {
        return;
    }

    parser.print();

    // it will generate LLVM IR code
    let mut ir = IrGen::new(&parser.ast);
    ir.gen_ir();
    let ir_source = ir.get_ir();

    fs::write("tests/output.ll", ir_source).unwrap();
}
