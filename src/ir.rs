use crate::{
    ast::{
        AssignStmt, Ast,
        Decl::{FuncDef, Var},
        Expr, FunctionDef, Stmt, VarStmt,
    },
    types::DataType,
};

pub struct IrGen<'i> {
    ast: &'i Ast,
    ir_source: String,
}

impl<'i> IrGen<'i> {
    pub fn new(ast: &'i Ast) -> Self {
        Self {
            ast,
            ir_source: String::new(),
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
                    let value = self.process_expr(expr, false);
                    self.push(format!("    ret {}", value).as_str());
                }
                Stmt::Var(vs) => self.process_vs(vs),
                Stmt::Assign(stmt) => self.process_assign_stmt(stmt),
            }
        }

        self.push("}");
    }

    /// variable statement
    fn process_vs(&mut self, vs: &VarStmt) {
        let dt = self.get_ir_type(vs.data_type);
        let name = self.get_var_name(&vs.name, vs.id);

        self.push(&format!("    {} = alloca {}", name, dt));

        match vs.value {
            Expr::Int32(value) => {
                self.push(&format!("    store i32 {}, ptr {}", value, name));
                let load_name = self.get_load_name(&vs.name, vs.id);
                self.push(&format!("    {} = load {}, ptr {}", load_name, dt, name));
            }
            _ => todo!("fuck you"),
        }
    }

    // NOTE: don't forget to use `load` variable name
    fn process_expr(&mut self, expr: &Expr, is_ptr: bool) -> String {
        match expr {
            Expr::Int32(value) => format!("i32 {}", value),
            Expr::String(_) => todo!(),
            Expr::Ident(expr) => {
                let name = self.get_load_name(&expr.name, expr.id);
                let dt = self.get_ir_type(expr.data_type.unwrap());
                return format!("{} {}", dt, name);
            }

            Expr::BinaryExpr(_) => todo!(),
            Expr::Empty => todo!(),
        }
    }

    fn process_assign_stmt(&mut self, stmt: &AssignStmt) {
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
}
