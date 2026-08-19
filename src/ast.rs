use std::fmt::Display;

use crate::types::DataType;

pub struct Ast {
    pub nodes: Vec<Node>,
    pub err: Option<String>,
}

pub enum Node {
    FuncDef(FunctionDef),
    Var(VarStmt),
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    Return(Expr),
    Var(VarStmt),
}

#[derive(Debug)]
pub enum Expr {
    Int32(i32),
}

#[allow(dead_code)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Block,
    pub return_type: DataType,
}

#[allow(dead_code)]
pub struct Param {
    pub name: String,
    pub p_type: DataType,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct VarStmt {
    pub data_type: DataType,
    pub name: String,
    pub expr: Expr,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int32(v) => write!(f, "{}", v),
        }
    }
}
