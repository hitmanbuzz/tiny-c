use std::collections::{HashMap, hash_map::Entry};

use crate::{
    ast::{Ast, Decl, Expr, FunctionDef, Stmt, VarStmt},
    types::DataType,
};

pub struct Semantic<'s> {
    ast: &'s Ast,
    scopes: Vec<HashMap<String, DataType>>,
}

impl<'s> Semantic<'s> {
    pub fn new(ast: &'s Ast) -> Self {
        Self {
            ast,
            scopes: Vec::new(),
        }
    }

    pub fn analyze(&mut self) {
        // global scope
        self.entry_scope();

        for decl in self.ast.decls.iter() {
            let result = match decl {
                Decl::Var(vs) => self.analyze_var(vs),
                Decl::FuncDef(fd) => self.analyze_fn(fd),
            };

            if let Err(e) = result {
                eprintln!("[SEMANTIC ERROR]: {}", e);
            }
        }

        self.exit_scope();
    }

    fn analyze_fn(&mut self, fd: &FunctionDef) -> Result<(), String> {
        // local scope
        self.entry_scope();

        for stmt in fd.body.stmts.iter() {
            match stmt {
                Stmt::Return(expr) => {
                    let expr_type = self.get_expr_type(expr)?;
                    if fd.return_type != expr_type {
                        return Err(format!(
                            "incompatible function return type and return stmt expr type: '{:?}' != '{:?}'",
                            fd.return_type, expr_type,
                        ));
                    }
                }
                Stmt::Var(stmt) => self.analyze_var(stmt)?,
            }
        }

        self.exit_scope();
        Ok(())
    }

    fn analyze_var(&mut self, vs: &VarStmt) -> Result<(), String> {
        let expr_type = self.get_expr_type(&vs.expr)?;
        if vs.data_type != expr_type {
            return Err(format!(
                "incompatible var data type and var expr type: '{:?}' != '{:?}'",
                vs.data_type, expr_type,
            ));
        }

        self.declare(vs.name.clone(), vs.data_type)?;
        Ok(())
    }

    fn get_expr_type(&self, expr: &Expr) -> Result<DataType, String> {
        match expr {
            Expr::Int32(_) => Ok(DataType::Int),
            Expr::String(_) => Ok(DataType::CharPtr),
            Expr::Ident(name) => self
                .lookup(name)
                .ok_or_else(|| format!("use of undeclared identifier: '{}'", name)),
            Expr::BinaryExpr(expr) => {
                let mut expr = *expr.clone();
                let left_expr_type = self.get_expr_type(&mut expr.left)?;
                let right_expr_type = self.get_expr_type(&mut expr.right)?;

                if left_expr_type != right_expr_type {
                    return Err(format!(
                        "operation of different expression data type not allowed: '{:?}' != '{:?}",
                        left_expr_type, right_expr_type,
                    ));
                }

                Ok(left_expr_type)
            }
            Expr::Empty => Ok(DataType::Void),
        }
    }

    fn declare(&mut self, name: String, data_type: DataType) -> Result<(), String> {
        let scope = self.scopes.last_mut().unwrap();
        match scope.entry(name) {
            Entry::Occupied(entry) => Err(format!("redefinition of '{}'", entry.key())),
            Entry::Vacant(entry) => {
                entry.insert(data_type);
                Ok(())
            }
        }
    }

    fn lookup(&self, name: &str) -> Option<DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(data_type) = scope.get(name) {
                return Some(*data_type);
            }
        }
        None
    }

    fn entry_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }
}
