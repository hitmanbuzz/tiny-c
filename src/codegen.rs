use std::{collections::HashMap, path::Path, process::exit};

use inkwell::{
    FloatPredicate, IntPredicate,
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, PointerValue},
};

use crate::{
    ast::{
        AssignStmt, Ast, BinaryExpr, BinaryOp, Block,
        Decl::{self, FuncDef, Var},
        Expr, FunctionDef, Stmt, VarStmt,
    },
    types::DataType,
};

struct AllocaInfo<'c> {
    ptr: PointerValue<'c>,
    ty: BasicTypeEnum<'c>,
}

struct Scope<'c> {
    allocas: HashMap<String, AllocaInfo<'c>>,
}

impl<'c> Scope<'c> {
    fn new() -> Self {
        Self {
            allocas: HashMap::new(),
        }
    }

    fn declare(&mut self, name: String, ptr: PointerValue<'c>, ty: BasicTypeEnum<'c>) {
        self.allocas.insert(name, AllocaInfo { ptr, ty });
    }

    fn lookup(&self, name: &str) -> Option<&AllocaInfo<'c>> {
        self.allocas.get(name)
    }
}

pub struct CodeGen<'c> {
    context: &'c Context,
    module: Module<'c>,
    builder: Builder<'c>,

    ast: &'c Ast,

    symbols: Vec<Scope<'c>>,
}

impl<'c> CodeGen<'c> {
    pub fn new(ast: &'c Ast, context: &'c Context, file_path: &Path) -> Self {
        let file_name = file_path
            .file_name()
            .expect("failed to get filename")
            .to_str()
            .expect("failed to convert filename to &str");
        let module = context.create_module(file_name);

        Self {
            context: context,
            module: module,
            builder: context.create_builder(),
            ast,
            symbols: Vec::new(),
        }
    }

    pub fn generate(&mut self) {
        self.gen_program();

        if let Err(err) = self.module.verify() {
            eprintln!("Module Verification failed: {:?}", err);
            exit(-1);
        }
    }

    pub fn save(&self, target_path: &str) {
        self.module.print_to_file(target_path).unwrap();
    }

    pub fn get_ir_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    fn gen_program(&mut self) {
        for decl in self.ast.decls.iter() {
            self.gen_decl(decl);
        }
    }

    fn gen_decl(&mut self, decl: &Decl) {
        match decl {
            FuncDef(fd) => self.gen_fn(fd),
            Var(stmt) => self.gen_var_stmt(stmt),
        }
    }

    fn gen_fn(&mut self, fd: &FunctionDef) {
        let fn_type = self.create_fn_type(fd.return_type, &[], false);
        let function = self.module.add_function(&fd.name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        self.gen_body(&fd.body);
    }

    fn gen_body(&mut self, body: &Block) {
        // FIX: function body should also be in the symbol table
        self.push_scope();

        for stmt in body.stmts.iter() {
            self.gen_stmt(stmt);
        }

        self.pop_scope();
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => self.gen_return_stmt(expr),
            Stmt::Var(stmt) => self.gen_var_stmt(stmt),
            Stmt::Assign(stmt) => self.gen_assign_stmt(stmt),
            Stmt::IfStmt(stmt) => todo!(),
        }
    }

    fn gen_var_stmt(&mut self, vs: &VarStmt) {
        if let Some(ty) = self.create_llvm_type(vs.data_type) {
            let ptr = self
                .builder
                .build_alloca(ty, &vs.name)
                .expect(format!("failed to build alloca: {}", &vs.name).as_str());

            let alloca_name = self.create_alloca_name(
                &vs.name,
                vs.id
                    .expect(format!("failed to get symbol id: {}", &vs.name).as_str()),
            );

            if let Some(value) = &vs.value {
                let result = self.gen_expr(value);
                self.builder
                    .build_store(ptr, result.0)
                    .expect(format!("failed to build store: {}", &vs.name).as_str());
            }

            self.declare(alloca_name, ptr, ty);
        } else {
            eprintln!(
                "invalid variable data type declaration: {:?}({})",
                vs.data_type,
                vs.name.as_str()
            );
            exit(-1);
        }
    }

    fn gen_assign_stmt(&self, stmt: &AssignStmt) {
        if let Expr::Ident(ident) = &stmt.target {
            let alloca_name = self.create_alloca_name(
                &ident.name,
                ident
                    .id
                    .expect(format!("failed to get symbol id: {}", &ident.name).as_str()),
            );

            let info = self
                .lookup(&alloca_name)
                .expect(format!("failed to lookup ident: {}", &ident.name).as_str());
            let value = self.gen_expr(&stmt.value);
            self.builder
                .build_store(info.ptr, value.0)
                .expect(format!("failed to build store: {}", &ident.name).as_str());
        } else {
            unreachable!("semantic analysis fuck up checking the LHS properly");
        }
    }

    fn gen_return_stmt(&self, expr: &Expr) {
        let result = self.gen_expr(expr);
        self.builder
            .build_return(Some(&result.0))
            .expect("failed to build return");
    }

    fn gen_expr(&self, expr: &Expr) -> (BasicValueEnum<'c>, DataType) {
        match expr {
            Expr::Int32(n) => {
                let llvm_ty = self.context.i32_type();
                return (llvm_ty.const_int(*n as u64, false).into(), DataType::Int32);
            }
            Expr::Float32(n) => {
                let llvm_ty = self.context.f32_type();
                return (llvm_ty.const_float(*n as f64).into(), DataType::Float32);
            }
            Expr::String(_) => todo!(),
            Expr::Bool(b) => {
                return (
                    self.context.bool_type().const_int(*b as u64, false).into(),
                    DataType::Bool,
                );
            }
            Expr::Ident(ix) => {
                let alloca_name = self.create_alloca_name(
                    &ix.name,
                    ix.id
                        .expect(format!("failed to get symbol id: {}", ix.name.as_str()).as_str()),
                );

                let info = self
                    .lookup(&alloca_name)
                    .expect(format!("failed to lookup ident: {}", ix.name.as_str()).as_str());
                return (
                    self.builder
                        .build_load(info.ty, info.ptr, &ix.name)
                        .expect("failed to create build load"),
                    ix.data_type.expect(
                        format!("failed to get identifer data type: {}", ix.name.as_str()).as_str(),
                    ),
                );
            }
            Expr::BinaryExpr(expr) => self.gen_binary_expr(expr),
        }
    }

    fn gen_binary_expr(&self, expr: &Box<BinaryExpr>) -> (BasicValueEnum<'c>, DataType) {
        let lhs = self.gen_expr(&expr.left);
        let rhs = self.gen_expr(&expr.right);
        self.create_binary_build(expr.op, lhs.0, rhs.0)
    }

    fn create_binary_build(
        &self,
        op: BinaryOp,
        lhs: BasicValueEnum<'c>,
        rhs: BasicValueEnum<'c>,
    ) -> (BasicValueEnum<'c>, DataType) {
        match (lhs, rhs) {
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = match op {
                    BinaryOp::Add => self.builder.build_int_add(l, r, "addtmp").unwrap(),
                    BinaryOp::Sub => self.builder.build_int_sub(l, r, "subtmp").unwrap(),
                    BinaryOp::Mul => self.builder.build_int_mul(l, r, "multmp").unwrap(),
                    BinaryOp::Div => self.builder.build_int_signed_div(l, r, "divtmp").unwrap(),
                    BinaryOp::Modulo => self.builder.build_int_signed_rem(l, r, "remtemp").unwrap(),
                    BinaryOp::Less => self
                        .builder
                        .build_int_compare(IntPredicate::SLT, l, r, "cmptmp")
                        .unwrap(),
                    BinaryOp::Greater => self
                        .builder
                        .build_int_compare(IntPredicate::SGT, l, r, "cmptmp")
                        .unwrap(),
                    BinaryOp::LessEqual => self
                        .builder
                        .build_int_compare(IntPredicate::SLE, l, r, "cmptmp")
                        .unwrap(),
                    BinaryOp::GreaterEqual => self
                        .builder
                        .build_int_compare(IntPredicate::SGE, l, r, "cmptmp")
                        .unwrap(),
                    BinaryOp::EqualEqual => self
                        .builder
                        .build_int_compare(IntPredicate::EQ, l, r, "cmptmp")
                        .unwrap(),
                    BinaryOp::NotEqual => self
                        .builder
                        .build_int_compare(IntPredicate::NE, l, r, "cmptmp")
                        .unwrap(),
                    _ => todo!(),
                };

                return (result.into(), DataType::Int32);
            }
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result: BasicValueEnum = match op {
                    BinaryOp::Add => self.builder.build_float_add(l, r, "addtmp").unwrap().into(),
                    BinaryOp::Sub => self.builder.build_float_sub(l, r, "subtmp").unwrap().into(),
                    BinaryOp::Mul => self.builder.build_float_mul(l, r, "multmp").unwrap().into(),
                    BinaryOp::Div => self.builder.build_float_div(l, r, "divtmp").unwrap().into(),
                    BinaryOp::Modulo => self
                        .builder
                        .build_float_rem(l, r, "remtemp")
                        .unwrap()
                        .into(),
                    BinaryOp::Less => self
                        .builder
                        .build_float_compare(FloatPredicate::OLT, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    BinaryOp::Greater => self
                        .builder
                        .build_float_compare(FloatPredicate::OGT, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    BinaryOp::LessEqual => self
                        .builder
                        .build_float_compare(FloatPredicate::OLE, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    BinaryOp::GreaterEqual => self
                        .builder
                        .build_float_compare(FloatPredicate::OGE, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    BinaryOp::EqualEqual => self
                        .builder
                        .build_float_compare(FloatPredicate::OEQ, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    BinaryOp::NotEqual => self
                        .builder
                        .build_float_compare(FloatPredicate::ONE, l, r, "cmptmp")
                        .unwrap()
                        .into(),
                    _ => todo!(),
                };
                return (result.into(), DataType::Float32);
            }
            _ => todo!(),
        }
    }

    fn push_scope(&mut self) {
        self.symbols.push(Scope::new());
    }

    fn pop_scope(&mut self) {
        self.symbols.pop();
    }

    fn declare(&mut self, name: String, ptr: PointerValue<'c>, ty: BasicTypeEnum<'c>) {
        self.symbols
            .last_mut()
            .expect("forgot to create/push scope")
            .declare(name, ptr, ty);
    }

    fn lookup(&self, name: &str) -> Option<&AllocaInfo<'c>> {
        for scope in self.symbols.iter().rev() {
            return scope.lookup(name);
        }
        None
    }

    fn create_llvm_type(&self, dt: DataType) -> Option<BasicTypeEnum<'c>> {
        match dt {
            DataType::Int32 => Some(self.context.i32_type().into()),
            DataType::Float32 => Some(self.context.f32_type().into()),
            DataType::Bool => Some(self.context.bool_type().into()),
            _ => None,
        }
    }

    fn create_fn_type(
        &self,
        data_type: DataType,
        param_types: &'c [BasicMetadataTypeEnum],
        is_var_args: bool,
    ) -> FunctionType<'c> {
        match data_type {
            DataType::Int32 => self.context.i32_type().fn_type(param_types, is_var_args),
            DataType::Bool => self.context.bool_type().fn_type(param_types, is_var_args),
            DataType::Void => self.context.void_type().fn_type(param_types, is_var_args),
            DataType::Float32 => self.context.f32_type().fn_type(param_types, is_var_args),
            DataType::String => todo!(),
        }
    }

    fn create_alloca_name(&self, name: &str, symbol_id: usize) -> String {
        format!("{}_{}", name, symbol_id)
    }
}
