mod ast;
mod codegen;
mod debug;
mod lexer;
mod parser;
mod semantic;
mod token;
mod types;

use clap::Parser;
use inkwell::context::Context;
use std::{fs, path::Path};

use crate::{codegen::CodeGen, lexer::Lexer, semantic::Semantic};

#[derive(clap::Parser)]
#[command(version)]
struct Cmd {
    /// source code file path
    #[arg(short, long, required = true)]
    source: Option<String>,

    /// final binary file path
    #[arg(short, long)]
    target: Option<String>,
}

fn main() {
    let cmd = Cmd::parse();
    let source = cmd.source.unwrap();
    let source_path = Path::new(&source);
    let source_code: String;

    if let Ok(content) = fs::read_to_string(source_path) {
        source_code = content;
    } else {
        eprintln!("failed to read file: {}", source_path.to_str().unwrap());
        return;
    }

    let mut lexer = Lexer::new(&source_code);
    lexer.tokenize();
    // lexer.print();

    let mut parser = parser::Parser::new(lexer.tokens);
    parser.parse();

    let mut sym = Semantic::new();
    let has_err = sym.analyze(&mut parser.ast);
    if has_err {
        return;
    }

    parser.print();

    let context = Context::create();
    let cg = CodeGen::new(&parser.ast, &context, source_path);
    cg.generate();
}
