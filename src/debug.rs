use crate::Stmt;
use crate::parser::Parser;

impl Parser {
    pub fn print(&self) {
        if !self.err_msg.is_empty() {
            eprintln!("[ERROR] {}", self.err_msg);
        } else {
            if let Some(ref f) = self.ast.f {
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
    }
}
