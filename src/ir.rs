use crate::{
    ast::{
        AssignStmt, Ast, BinaryExpr, BinaryOp,
        Decl::{FuncDef, Var},
        Expr, FunctionDef, Stmt, VarStmt,
    },
    types::DataType,
};

pub struct IrGen<'i> {
    ast: &'i Ast,
    ir_source: String,
    counter: usize,
}

impl<'i> IrGen<'i> {
    pub fn new(ast: &'i Ast) -> Self {
        Self {
            ast,
            ir_source: String::new(),
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

    /// function definition
    fn process_fd(&mut self, fd: &FunctionDef) {
        let rt = self.get_ir_type(fd.return_type);

        self.push(&format!("define {} @{}() {{", rt, fd.name));
        self.push("entry:");

        for stmt in fd.body.stmts.iter() {
            match stmt {
                Stmt::Return(expr) => {
                    let (dt, value) = self.process_expr("", expr, false, 0);
                    self.push(format!("    ret {} {}", dt, value).as_str());
                }
                Stmt::Var(vs) => self.process_vs(vs),
                Stmt::Assign(stmt) => self.process_assign_stmt(stmt),
            }
        }

        self.push("}");
    }

    /// variable statement
    fn process_vs(&mut self, vs: &VarStmt) {
        let alloca_dt = self.get_ir_type(vs.data_type);
        let alloca_name = self.get_var_name(&vs.name, vs.id);

        self.push(&format!("    {} = alloca {}", alloca_name, alloca_dt));

        let (dt, value) = self.process_expr(&alloca_name, &vs.value, false, 0);
        let load_name = self.get_load_name(&vs.name, vs.id);
        self.push(&format!("    store {} {}, ptr {}", dt, value, alloca_name));
        self.push(&format!(
            "    {} = load {}, ptr {}\n",
            load_name, dt, alloca_name
        ));
    }

    /// (DataType, Value)
    fn process_expr(
        &mut self,
        name: &str,
        expr: &Expr,
        is_ptr: bool,
        counter: usize,
    ) -> (String, String) {
        match expr {
            Expr::Int32(value) => return ("i32".to_string(), value.to_string()),
            Expr::String(_) => todo!(),
            Expr::Ident(expr) => {
                let name = self.get_load_name(&expr.name, expr.id);
                let dt = self.get_ir_type(expr.data_type.unwrap());
                return (dt.to_string(), name);
            }

            Expr::BinaryExpr(expr) => {
                let result = self.process_binary_expr(name, expr, is_ptr, counter);
                return result;
            }
            Expr::Empty => todo!(),
        }
    }

    fn process_binary_expr(
        &mut self,
        name: &str,
        expr: &Box<BinaryExpr>,
        _is_ptr: bool,
        counter: usize,
    ) -> (String, String) {
        let op = self.get_op_type(expr.op);
        let lhs = self.process_expr(name, &expr.left, false, counter + 1);
        let rhs = self.process_expr(name, &expr.right, false, counter + 1);

        let temp = self.temp_name(name, op);
        self.push(format!("    {} = {} {} {}, {}", temp, op, lhs.0, lhs.1, rhs.1).as_str());

        return (lhs.0, temp);
    }

    fn process_assign_stmt(&mut self, _stmt: &AssignStmt) {
        todo!()
    }

    fn push(&mut self, source: &str) {
        self.ir_source.push_str(format!("{}\n", source).as_str());
    }

    fn get_var_name(&self, name: &str, id: Option<usize>) -> String {
        return format!("%{}_{}", name, id.unwrap());
    }

    /// this is to create a IR variable for `load` since we can't simply use name like
    ///
    /// %x_0 -> alloca (ptr) when we want to access the value
    ///
    /// It will create %x_load_0 for storing the value so that we can access it
    fn get_load_name(&self, name: &str, id: Option<usize>) -> String {
        return format!("%{}_value_{}", name, id.unwrap());
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
        }
    }

    fn temp_name(&mut self, prefix: &str, op: &str) -> String {
        let n = self.counter;
        self.counter += 1;
        format!("{}_{}_{}", prefix, op, n)
    }
}
