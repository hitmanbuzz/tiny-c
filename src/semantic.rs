use crate::{
    ast::{Ast, Decl, Expr, FunctionDef, Stmt, VarStmt},
    types::DataType,
};

pub struct Symantic<'s> {
    ast: &'s Ast,
}

impl<'s> Symantic<'s> {
    pub fn new(ast: &'s Ast) -> Self {
        Self { ast }
    }

    pub fn analyze(&self) {
        for decl in self.ast.decls.iter() {
            match decl {
                Decl::FuncDef(fd) => {
                    if let Err(e) = self.analyze_fn(fd) {
                        eprintln!("[ERROR]: {}", e);
                    }
                }
                Decl::Var(vs) => {
                    if let Err(e) = self.analyze_var(vs) {
                        eprintln!("[ERROR]: {}", e);
                    }
                }
            }
        }
    }

    fn analyze_fn(&self, fd: &FunctionDef) -> Result<(), String> {
        for stmt in fd.body.stmts.iter() {
            match stmt {
                Stmt::Return(expr) => {
                    let expr_type = self.get_expr_type(expr)?;
                    if fd.return_type != expr_type {
                        return Err(format!(
                            "incompatible function return type and return stmt expr: `{:?}` != `{:?}`",
                            fd.return_type, expr_type,
                        ));
                    }
                }
                Stmt::Var(var_stmt) => todo!(),
            }
        }

        Ok(())
    }

    fn analyze_var(&self, vs: &VarStmt) -> Result<(), String> {
        let expr_type = self.get_expr_type(&vs.expr)?;
        if vs.data_type != expr_type {
            return Err(format!(
                "incompatible var return type and var expr: `{:?} != `{:?}",
                vs.data_type, expr_type,
            ));
        }
        Ok(())
    }

    fn get_expr_type(&self, expr: &Expr) -> Result<DataType, String> {
        match expr {
            Expr::Int32(_) => Ok(DataType::Int),
            Expr::String(_) => Ok(DataType::CharPtr),
            Expr::Ident(_) => todo!(),
            Expr::BinaryExpr(_) => todo!(),
            Expr::Empty => Ok(DataType::Void),
        }
    }
}
