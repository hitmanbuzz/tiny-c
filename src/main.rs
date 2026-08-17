mod ast;
mod lexer;
mod parser;
mod token;
mod types;

use crate::{
    ast::Stmt,
    lexer::{Lexer, LexerError},
    parser::Parser,
};

fn main() {
    let source = r#"
        int main() {
            return 69;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    for err in lexer.errors.iter() {
        match err {
            LexerError::IDENT_START_WITH_NUM(token) => {
                eprintln!("identifier cannot start with a number: `{}`", token)
            }
            LexerError::NUM_HAS_MORE_DOTS(token) => {
                eprintln!("number contains more than 1 dot: `{}`", token)
            }
            LexerError::BAD_STRING(token) => {
                eprintln!("string doesn't have a end quotation: `{}`", token)
            }
        }
    }

    // println!("{:#?}", lexer.tokens);

    let mut parser = Parser::new(lexer.tokens);
    let program = parser.parse();
    match program {
        Ok(p) => {
            if let Some(f) = p.f {
                println!("func_return_type ({:?})", f.return_type);
                println!("func_name ({})", f.name);
                for stmt in f.body.stmts.iter() {
                    match stmt {
                        Stmt::RETURN(expr) => println!("    func_return ({})", expr),
                        Stmt::VARIABLE(data_type, name, expr) => {
                            println!(
                                "    var_type ({:?}) | var_name ({}) | var_value ({})",
                                data_type, name, expr
                            )
                        }
                    }
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
