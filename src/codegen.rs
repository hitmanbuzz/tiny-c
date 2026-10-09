use std::{path::Path, process::exit};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    types::{BasicMetadataTypeEnum, FunctionType},
};

use crate::{
    ast::{
        Ast, Block,
        Decl::{self, FuncDef, Var},
        FunctionDef, Stmt, VarStmt,
    },
    types::DataType,
};

pub struct CodeGen<'c> {
    context: &'c Context,
    module: Module<'c>,
    builder: Builder<'c>,

    ast: &'c Ast,
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
        }
    }

    pub fn generate(&self) {
        self.gen_program();

        if let Err(err) = self.module.verify() {
            eprintln!("Module Verification failed: {:?}", err);
            exit(-1);
        }
    }

    fn gen_program(&self) {
        for decl in self.ast.decls.iter() {
            self.gen_decl(decl);
        }
    }

    fn gen_decl(&self, decl: &Decl) {
        match decl {
            FuncDef(fd) => self.gen_fn(fd),
            Var(vs) => self.gen_vs(vs),
        }
    }

    fn gen_fn(&self, fd: &FunctionDef) {
        let fn_type = self.create_fn_type(fd.return_type, &[], false);
        let function = self.module.add_function(&fd.name, fn_type, None);
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
        self.gen_body(&fd.body);
    }

    fn gen_body(&self, body: &Block) {
        for stmt in body.stmts.iter() {
            self.gen_stmt(stmt);
        }
    }

    fn gen_stmt(&self, stmt: &Stmt) {
        // TODO: time to make another SymbolTable 😭
        match stmt {
            Stmt::Return(expr) => todo!(),
            Stmt::Var(stmt) => todo!(),
            Stmt::Assign(stmt) => todo!(),
            Stmt::IfStmt(stmt) => todo!(),
        }
    }

    fn gen_vs(&self, vs: &VarStmt) {}

    fn create_fn_type(
        &self,
        data_type: DataType,
        param_types: &'c [BasicMetadataTypeEnum],
        is_var_args: bool,
    ) -> FunctionType<'c> {
        match data_type {
            DataType::Int32 => self.context.i32_type().fn_type(param_types, is_var_args),
            DataType::Bool => self.context.bool_type().fn_type(param_types, is_var_args),
            DataType::String => todo!(),
            DataType::Void => self.context.void_type().fn_type(param_types, is_var_args),
        }
    }
}
