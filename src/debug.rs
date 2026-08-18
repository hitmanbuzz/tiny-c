use crate::ast::{
    Node::{FuncDef, Var},
    Stmt,
};
use crate::lexer::Lexer;
use crate::parser::Parser;

impl<'l> Lexer<'l> {
    pub fn print(&self) {
        println!("{:#?}\n", self.tokens);
    }
}

impl Parser {
    pub fn print(&self) {
        match &self.ast.err {
            Some(err) => eprintln!("[ERROR] {}", err.as_str()),
            None => {
                for node in self.ast.nodes.iter() {
                    match node {
                        FuncDef(f) => {
                            println!("func_return_type ({:?})", f.return_type);
                            println!("func_name ({})", f.name);
                            for stmt in f.body.stmts.iter() {
                                match stmt {
                                    Stmt::Return(expr) => println!("    func_return ({})", expr),
                                    Stmt::Var(stmt) => todo!("{:?}", stmt),
                                }
                            }
                        }
                        Var(v) => todo!("{:?}", v),
                    }
                }
            }
        }
    }
}
