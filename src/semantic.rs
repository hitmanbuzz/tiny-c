use std::collections::{HashMap, hash_map::Entry};

use crate::{
    ast::{AssignStmt, Ast, Decl, Expr, FunctionDef, Stmt, VarStmt},
    types::DataType,
};

#[derive(Clone, Copy)]
struct ScopeData {
    id: Option<usize>,
    data_type: DataType,
}

impl ScopeData {
    fn new(id: Option<usize>, data_type: DataType) -> Self {
        Self { id, data_type }
    }
}

pub struct Semantic {
    scopes: Vec<HashMap<String, ScopeData>>,
    id_counter: usize,
}

impl Semantic {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn analyze(&mut self, ast: &mut Ast) {
        // global scope
        self.entry_scope();

        for decl in &mut ast.decls {
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
                Stmt::Var(stmt) => self.analyze_var(stmt)?,
                Stmt::Assign(stmt) => self.analyze_assign(stmt)?,
            }
        }

        self.exit_scope();
        Ok(())
    }

    fn analyze_var(&mut self, vs: &mut VarStmt) -> Result<(), String> {
        let scope = self.get_scope_data(&mut vs.value)?;
        if vs.data_type != scope.data_type {
            return Err(format!(
                "incompatible var data type and var expr type: '{:?}' != '{:?}'",
                vs.data_type, scope.data_type,
            ));
        }

        let id = self.declare(vs.name.clone(), vs.data_type)?;
        vs.id = Some(id);
        Ok(())
    }

    fn analyze_assign(&mut self, stmt: &mut AssignStmt) -> Result<(), String> {
        let target_scope = self.get_scope_data(&mut stmt.target)?;
        let value_scope = self.get_scope_data(&mut stmt.target)?;

        if target_scope.data_type != value_scope.data_type {
            return Err(format!(
                "incompatible asssign (value) data type with the target data type: '{:?}' != '{:?}'",
                target_scope.data_type, value_scope.data_type
            ));
        }

        Ok(())
    }

    fn get_scope_data(&self, expr: &mut Expr) -> Result<ScopeData, String> {
        match expr {
            Expr::Int32(_) => Ok(ScopeData::new(None, DataType::Int)),
            Expr::String(_) => Ok(ScopeData::new(None, DataType::CharPtr)),
            Expr::Ident(name, id) => {
                let scope = self
                    .lookup(name)
                    .ok_or_else(|| format!("use of undeclared identifier: '{}'", name))?;
                *id = scope.id;
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
            Expr::Empty => Ok(ScopeData::new(None, DataType::Void)),
        }
    }

    fn declare(&mut self, name: String, data_type: DataType) -> Result<usize, String> {
        let scope = self.scopes.last_mut().unwrap();
        match scope.entry(name) {
            Entry::Occupied(entry) => Err(format!("redefinition of '{}'", entry.key())),
            Entry::Vacant(entry) => {
                let id = self.id_counter;
                self.id_counter += 1;
                entry.insert(ScopeData::new(Some(id), data_type));
                Ok(id)
            }
        }
    }

    fn lookup(&self, name: &str) -> Option<ScopeData> {
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
