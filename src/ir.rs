use std::collections::{HashMap, hash_map::Entry};

use crate::{
    ast::{
        AssignStmt, Ast, BinaryExpr, BinaryOp, Block,
        Decl::{FuncDef, Var},
        Expr, FunctionDef, IfStmt, Stmt, VarStmt,
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

    pub fn get_ir(&self) -> &str {
        return self.ir_source.as_str();
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
            self.process_stmt(stmt);
        }

        self.push("}");
    }

    fn process_stmt(&mut self, stmt: &Stmt) {
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
            Stmt::IfStmt(stmt) => self.process_if_stmt(stmt),
        }
    }

    fn process_vs(&mut self, stmt: &VarStmt) {
        let alloca_dt = self.get_ir_type(stmt.data_type);
        let alloca_name = self.get_alloca_name(&stmt.name, stmt.id);

        // CLONE: try to fix this damn clone thing
        self.update_register(alloca_name.clone());

        self.push(&format!("    {} = alloca {}", alloca_name, alloca_dt));

        let (dt, value) = self.process_expr(&alloca_name, &stmt.value, 0);
        if dt.is_empty() && value.is_empty() {
            return;
        }

        self.push(&format!("    store {} {}, ptr {}", dt, value, alloca_name));
        self.update_register(alloca_name.clone());
    }

    fn process_if_stmt(&mut self, stmt: &IfStmt) {
        let mut has_else = false;
        if stmt.else_stmt.is_some() {
            has_else = true;
        }

        let mut next_branch = "after";
        for (count, branch) in stmt.branches.iter().enumerate() {
            if count == stmt.branches.len() - 1 {
                match has_else {
                    true => next_branch = "else",
                    false => {}
                }
            }

            let (dt, value) = self.process_expr("%comp", &branch.cond_expr, 0);
            let then_branch = format!("then_{}", count);
            let else_branch = format!("{}_{}", next_branch, count);

            self.push(
                format!(
                    "    br {} {}, label %{}, label %{}",
                    dt, value, then_branch, else_branch
                )
                .as_str(),
            );

            self.process_if_branch(&then_branch, &branch.body, &else_branch);
        }

        // match has_else {
        //     true => {}
        //     false => {}
        // }
    }

    fn process_if_branch(&mut self, branch: &str, branch_body: &Block, next_branch: &str) {
        self.push(format!("{}:", branch).as_str());

        for stmt in branch_body.stmts.iter() {
            self.process_stmt(stmt);
        }
        self.push(format!("    br label %{}", next_branch).as_str());
        self.push(format!("{}:", next_branch).as_str());
    }

    /// param (lhs alloca var name, rhs expr, counter for binary expr)
    ///
    /// return (DataType, Value/Register)
    fn process_expr(&mut self, name: &str, expr: &Expr, counter: usize) -> (String, String) {
        match expr {
            Expr::Int32(value) => ("i32".to_string(), value.to_string()),
            Expr::String(_) => todo!(),
            Expr::Bool(value) => ("i1".to_string(), value.to_string()),
            Expr::Ident(expr) => {
                let alloca_name = self.get_alloca_name(&expr.name, expr.id);
                let load_name = self.get_load_name(&expr.name, expr.id);
                let dt =
                    self.get_ir_type(expr.data_type.expect("failed to get ident expr data type"));
                self.push(format!("    {} = load {}, ptr {}", load_name, dt, alloca_name).as_str());
                let dt = self.get_ir_type(expr.data_type.unwrap());
                return (dt.to_string(), load_name);
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
        // kinda dumb way but it helps avoiding reallocation
        let mut op = String::with_capacity(12);
        op.push_str(self.get_op_type(expr.op));
        let lhs = self.process_expr(name, &expr.left, counter + 1);
        let rhs = self.process_expr(name, &expr.right, counter + 1);
        let mut is_bool = false;

        let temp = self.temp_name(name, op.as_str());

        match expr.op {
            BinaryOp::Less | BinaryOp::Greater => {
                op.insert_str(0, "icmp ");
                is_bool = true;
            }
            _ => {}
        }

        self.push(format!("    {} = {} {} {}, {}", temp, op, lhs.0, lhs.1, rhs.1).as_str());

        match is_bool {
            true => return (String::from("i1"), temp),
            false => return (lhs.0, temp),
        }
    }

    fn process_assign_stmt(&mut self, stmt: &AssignStmt) {
        if let Expr::Ident(expr) = &stmt.target {
            let alloca_name = self.get_alloca_name(&expr.name, expr.id);

            // FIX: try to fix this damn clone thing
            let (dt, value) = self.process_expr(&alloca_name, &stmt.value, 0);
            self.update_register(alloca_name.clone());
            // let load_name = self.get_load_name(&expr.name, expr.id);

            self.push(format!("    store {} {}, ptr {}", dt, value, alloca_name).as_str());
            // self.push(format!("    {} = load {}, ptr {}", load_name, dt, alloca_name).as_str());
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

    // convert to LLVM IR data type
    fn get_ir_type(&self, dt: DataType) -> &'i str {
        match dt {
            DataType::Int32 => "i32",
            DataType::String => "string",
            DataType::Void => "void",
            DataType::Bool => "i1",
        }
    }

    fn get_op_type(&self, op: BinaryOp) -> &'i str {
        match op {
            BinaryOp::Add => "add",
            BinaryOp::Sub => "sub",
            BinaryOp::Mul => "mul",
            BinaryOp::Div => "sdiv",
            BinaryOp::Modulo => "srem",
            BinaryOp::Less => "slt",
            BinaryOp::Greater => "sgt",
            BinaryOp::LessEqual => todo!(),
            BinaryOp::NotEqual => todo!(),
            BinaryOp::GreaterEqual => todo!(),
            BinaryOp::EqualEqual => todo!(),
            BinaryOp::And => todo!(),
            BinaryOp::Or => todo!(),
            BinaryOp::BitOr => todo!(),
            BinaryOp::BitAnd => todo!(),
            BinaryOp::BitXor => todo!(),
            BinaryOp::BitLS => todo!(),
            BinaryOp::BitRS => todo!(),
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
