use std::collections::{HashMap, hash_map::Entry};

use crate::{
    ast::{AssignStmt, Ast, BinaryOp, Decl, Expr, FunctionDef, IfStmt, Stmt, VarStmt},
    types::DataType,
};

#[derive(Debug, Clone, Copy)]
struct ExprData {
    id: Option<usize>,
    data_type: DataType,
}

impl ExprData {
    fn new(id: Option<usize>, data_type: DataType) -> Self {
        Self { id, data_type }
    }
}

#[derive(Debug)]
pub struct Semantic {
    scopes: Vec<HashMap<String, ExprData>>,
    id_counter: usize,
}

impl Semantic {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn analyze(&mut self, ast: &mut Ast) -> bool {
        let mut has_err = false;
        // global scope
        self.entry_scope();

        for decl in &mut ast.decls {
            let result = match decl {
                Decl::Var(vs) => self.analyze_var(vs, true),
                Decl::FuncDef(fd) => self.analyze_fn(fd),
            };

            if let Err(e) = result {
                eprintln!("[SEMANTIC ERROR]: {}", e);
                has_err = true;
            }
        }

        self.exit_scope();
        return has_err;
    }

    fn analyze_fn(&mut self, fd: &mut FunctionDef) -> Result<(), String> {
        // local scope
        self.entry_scope();

        for stmt in fd.body.stmts.iter_mut() {
            match stmt {
                Stmt::Return(expr) => {
                    let scope = self.get_scope_data(expr)?;
                    if fd.return_type != scope.data_type {
                        return Err(format!(
                            "incompatible function return type and return stmt expr type: '{:?}' != '{:?}'",
                            fd.return_type, scope.data_type,
                        ));
                    }
                }
                Stmt::Var(stmt) => self.analyze_var(stmt, false)?,
                Stmt::Assign(stmt) => self.analyze_assign(stmt)?,
                Stmt::IfStmt(stmt) => self.analyze_if_stmt(stmt)?,
            }
        }

        self.exit_scope();
        Ok(())
    }

    fn analyze_var(&mut self, vs: &mut VarStmt, is_global: bool) -> Result<(), String> {
        let scope = self.analyze_expr(&mut vs.value)?;

        if vs.data_type != scope.data_type && scope.data_type != DataType::Void {
            return Err(format!(
                "incompatible var data type and var expr type: '{:?}' != '{:?}'",
                vs.data_type, scope.data_type,
            ));
        }

        let id = self.declare(vs.name.clone(), vs.data_type)?;
        vs.id = Some(id);
        vs.is_global = is_global;
        Ok(())
    }

    fn analyze_assign(&mut self, stmt: &mut AssignStmt) -> Result<(), String> {
        let target_scope = self.get_scope_data(&mut stmt.target)?;
        let value_scope = self.get_scope_data(&mut stmt.value)?;
        if target_scope.data_type != value_scope.data_type {
            return Err(format!(
                "incompatible asssign (value) data type with the target data type: '{:?}' != '{:?}'",
                target_scope.data_type, value_scope.data_type
            ));
        }

        Ok(())
    }

    fn analyze_if_stmt(&mut self, stmt: &mut IfStmt) -> Result<(), String> {
        todo!()
    }

    fn analyze_expr(&mut self, expr: &mut Expr) -> Result<ExprData, String> {
        match expr {
            Expr::Int32(_) => Ok(ExprData::new(None, DataType::Int32)),
            Expr::String(_) => Ok(ExprData::new(None, DataType::String)),

            Expr::Ident(expr) => {
                let scope = self
                    .lookup(&expr.name)
                    .ok_or_else(|| format!("use of undeclared identifier: '{}'", expr.name))?;

                expr.id = scope.id;
                expr.data_type = Some(scope.data_type);
                Ok(scope)
            }

            Expr::BinaryExpr(expr) => {
                let lhs = self.analyze_expr(&mut expr.left)?;
                let rhs = self.analyze_expr(&mut expr.right)?;
                let result_type =
                    self.analyze_binary_expr(expr.op, lhs.data_type, rhs.data_type)?;

                Ok(ExprData::new(None, result_type))
            }

            // FIX: remove `Empty` type from Expr and use instead Option<T> where `T` is expr
            Expr::Empty => Ok(ExprData::new(None, DataType::Void)),
        }
    }

    /// NOTE: the error is kinda plain and does't contain much info but I am too lazy to fix that
    fn analyze_binary_expr(
        &self,
        op: BinaryOp,
        lhs: DataType,
        rhs: DataType,
    ) -> Result<DataType, String> {
        match op {
            // arithmetic
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Modulo => {
                if lhs == DataType::Int32 && rhs == DataType::Int32 {
                    Ok(DataType::Int32)
                } else {
                    Err(format!(
                        "operator {:?} requires both operands to be Int32, found LHS({:?}) and RHS({:?})",
                        op, lhs, rhs
                    ))
                }
            }

            // relational
            BinaryOp::Less | BinaryOp::Greater | BinaryOp::LessEqual | BinaryOp::GreaterEqual => {
                if lhs == rhs {
                    Ok(DataType::Bool)
                } else {
                    Err(format!(
                        "LHS and RHS data type are not equal: LHS({:?}) != RHS({:?})",
                        lhs, rhs
                    ))
                }
            }

            // equality (relational)
            BinaryOp::EqualEqual | BinaryOp::NotEqual => {
                if lhs == rhs {
                    Ok(DataType::Bool)
                } else {
                    Err(format!(
                        "LHS and RHS data type are not equal: LHS({:?}) != RHS({:?})",
                        lhs, rhs
                    ))
                }
            }

            // logical
            BinaryOp::And | BinaryOp::Or => {
                if lhs == DataType::Bool && rhs == DataType::Bool {
                    Ok(DataType::Bool)
                } else {
                    Err(format!(
                        "operator {:?} requires both operands to be Bool, found LHS({:?}) and RHS({:?})",
                        op, lhs, rhs
                    ))
                }
            }

            // bitwise
            BinaryOp::BitOr | BinaryOp::BitAnd | BinaryOp::BitXor => {
                if lhs == DataType::Int32 && rhs == DataType::Int32 {
                    Ok(DataType::Int32)
                } else {
                    Err(format!(
                        "operator {:?} requires both operands to be Int32, found LHS({:?}) and RHS({:?})",
                        op, lhs, rhs
                    ))
                }
            }

            // bit shifts
            BinaryOp::BitLS | BinaryOp::BitRS => {
                if lhs == DataType::Int32 && rhs == DataType::Int32 {
                    Ok(DataType::Int32)
                } else {
                    Err(format!(
                        "operator {:?} requires both operands to be Int32, found LHS({:?}) and RHS({:?})",
                        op, lhs, rhs
                    ))
                }
            }
        }
    }

    fn get_scope_data(&mut self, expr: &mut Expr) -> Result<ExprData, String> {
        match expr {
            Expr::Int32(_) => Ok(ExprData::new(None, DataType::Int32)),
            Expr::String(_) => Ok(ExprData::new(None, DataType::String)),
            Expr::Ident(expr) => {
                let scope = self
                    .lookup(&expr.name)
                    .ok_or_else(|| format!("use of undeclared identifier: '{}'", expr.name))?;
                expr.id = scope.id;
                expr.data_type = Some(scope.data_type);
                return Ok(scope);
            }
            Expr::BinaryExpr(expr) => {
                let left_scope = self.get_scope_data(&mut expr.left)?;
                let right_scope = self.get_scope_data(&mut expr.right)?;

                if left_scope.data_type != right_scope.data_type {
                    return Err(format!(
                        "operation of different expression data type not allowed: '{:?}' != '{:?}",
                        left_scope.data_type, right_scope.data_type,
                    ));
                }

                Ok(left_scope)
            }
            Expr::Empty => {
                return Ok(ExprData::new(None, DataType::Void));
            }
        }
    }

    fn declare(&mut self, name: String, data_type: DataType) -> Result<usize, String> {
        let scope = self.scopes.last_mut().unwrap();
        match scope.entry(name) {
            Entry::Occupied(entry) => Err(format!("redefinition of '{}'", entry.key())),
            Entry::Vacant(entry) => {
                let id = self.id_counter;
                self.id_counter += 1;
                entry.insert(ExprData::new(Some(id), data_type));
                Ok(id)
            }
        }
    }

    fn lookup(&self, name: &str) -> Option<ExprData> {
        for scope in self.scopes.iter().rev() {
            if let Some(s) = scope.get(name) {
                return Some(*s);
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
