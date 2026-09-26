use std::collections::{HashMap, hash_map::Entry};

use crate::{
    ast::{
        AssignStmt, Ast, BinaryExpr, BinaryOp,
        Decl::{FuncDef, Var},
        Expr, FunctionDef, Stmt, VarStmt,
    },
    types::DataType,
};

#[derive(Debug)]
struct Register {
    curr_c: usize,
    next_c: usize,
}

pub struct IrGen<'i> {
    ast: &'i Ast,
    ir_source: String,
    registers: HashMap<String, Register>,
    counter: usize,
}

impl<'i> IrGen<'i> {
    pub fn new(ast: &'i Ast) -> Self {
        Self {
            ast,
            ir_source: String::new(),
            registers: HashMap::new(),
            counter: 0,
        }
    }

    pub fn gen_ir(&mut self) {
        self.process_ast();
    }

    pub fn get_ir(self) -> String {
        return self.ir_source;
    }

    fn process_ast(&mut self) {
        for decl in self.ast.decls.iter() {
            match decl {
                FuncDef(fd) => self.process_fd(fd),
                Var(vs) => self.process_vs(vs),
            }
        }
    }

    fn process_fd(&mut self, fd: &FunctionDef) {
        self.counter = 0;
        let rt = self.get_ir_type(fd.return_type);

        self.push(&format!("define {} @{}() {{", rt, fd.name));
        self.push("entry:");

        for stmt in fd.body.stmts.iter() {
            match stmt {
                Stmt::Return(expr) => {
                    let (dt, value) = self.process_expr("", expr, 0);
                    if dt.is_empty() && value.is_empty() {
                        self.push("    ret void");
                    } else {
                        self.push(format!("    ret {} {}", dt, value).as_str());
                    }
                }
                Stmt::Var(vs) => self.process_vs(vs),
                Stmt::Assign(stmt) => self.process_assign_stmt(stmt),
                Stmt::IfStmt(if_stmt) => todo!(),
            }
        }

        self.push("}");
    }

    fn process_vs(&mut self, vs: &VarStmt) {
        let alloca_dt = self.get_ir_type(vs.data_type);
        let alloca_name = self.get_alloca_name(&vs.name, vs.id);

        // CLONE: try to fix this damn clone thing
        self.update_register(alloca_name.clone());

        self.push(&format!("    {} = alloca {}", alloca_name, alloca_dt));

        let (dt, value) = self.process_expr(&alloca_name, &vs.value, 0);
        if dt.is_empty() && value.is_empty() {
            return;
        }

        let load_name = self.get_load_name(&vs.name, vs.id);
        self.push(&format!("    store {} {}, ptr {}", dt, value, alloca_name));
        self.push(&format!(
            "    {} = load {}, ptr {}",
            load_name, dt, alloca_name
        ));
        self.update_register(alloca_name.clone());
    }

    /// param (lhs alloca var name, rhs expr, counter for binary expr)
    ///
    /// return (DataType, Value/Register)
    fn process_expr(&mut self, name: &str, expr: &Expr, counter: usize) -> (String, String) {
        match expr {
            Expr::Int32(value) => return ("i32".to_string(), value.to_string()),
            Expr::String(_) => todo!(),
            Expr::Ident(expr) => {
                let name = self.get_load_name(&expr.name, expr.id);
                let dt = self.get_ir_type(expr.data_type.unwrap());
                return (dt.to_string(), name);
            }
            Expr::BinaryExpr(expr) => {
                let result = self.process_binary_expr(name, expr, counter);
                return result;
            }
            Expr::Empty => {
                return ("".to_string(), "".to_string());
            }
        }
    }

    /// return (DataType, Value/Register)
    fn process_binary_expr(
        &mut self,
        name: &str,
        expr: &Box<BinaryExpr>,
        counter: usize,
    ) -> (String, String) {
        let op = self.get_op_type(expr.op);
        let lhs = self.process_expr(name, &expr.left, counter + 1);
        let rhs = self.process_expr(name, &expr.right, counter + 1);

        let temp = self.temp_name(name, op);
        self.push(format!("    {} = {} {} {}, {}", temp, op, lhs.0, lhs.1, rhs.1).as_str());

        return (lhs.0, temp);
    }

    fn process_assign_stmt(&mut self, stmt: &AssignStmt) {
        if let Expr::Ident(expr) = &stmt.target {
            let alloca_name = self.get_alloca_name(&expr.name, expr.id);

            // FIX: try to fix this damn clone thing
            let (dt, value) = self.process_expr(&alloca_name, &stmt.value, 0);
            self.update_register(alloca_name.clone());
            let load_name = self.get_load_name(&expr.name, expr.id);

            self.push(format!("    store {} {}, ptr {}", dt, value, alloca_name).as_str());
            self.push(format!("    {} = load {}, ptr {}", load_name, dt, alloca_name).as_str());
        }
    }

    fn push(&mut self, source: &str) {
        self.ir_source.push_str(format!("{}\n", source).as_str());
    }

    fn get_alloca_name(&self, name: &str, id: Option<usize>) -> String {
        return format!("%{}_{}", name, id.unwrap());
    }

    fn get_load_name(&self, name: &str, id: Option<usize>) -> String {
        let alloca_name = format!(
            "%{}_{}",
            name,
            id.expect(format!("failed to get symbol id for: {}", name).as_str())
        );
        let r = self
            .registers
            .get(&alloca_name)
            .expect(format!("failed to get register: {}", alloca_name.as_str()).as_str());
        return format!("{}_value_{}", alloca_name, r.curr_c);
    }

    fn get_ir_type(&self, dt: DataType) -> &'i str {
        match dt {
            DataType::Int => "i32",
            DataType::CharPtr => "char*",
            DataType::Void => "void",
        }
    }

    fn get_op_type(&self, op: BinaryOp) -> &'i str {
        match op {
            BinaryOp::Add => "add",
            BinaryOp::Sub => "sub",
            BinaryOp::Mul => "mul",
            BinaryOp::Div => "sdiv",
            BinaryOp::Modulo => "srem",
            BinaryOp::Less => todo!(),
            BinaryOp::Greater => todo!(),
            BinaryOp::And => todo!(),
            BinaryOp::Or => todo!(),
        }
    }

    fn update_register(&mut self, alloca_name: String) {
        match self.registers.entry(alloca_name) {
            Entry::Occupied(mut e) => {
                e.get_mut().curr_c = e.get().next_c;
                e.get_mut().next_c += 1;
            }
            Entry::Vacant(e) => {
                e.insert(Register {
                    curr_c: 0,
                    next_c: 0,
                });
            }
        }
    }

    fn temp_name(&mut self, prefix: &str, op: &str) -> String {
        let n = self.counter;
        self.counter += 1;
        format!("{}_{}_{}", prefix, op, n)
    }
}
