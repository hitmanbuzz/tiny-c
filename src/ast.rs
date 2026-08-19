use std::fmt::Display;

use crate::types::DataType;

#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub err: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    FuncDef(FunctionDef),
    Var(VarStmt),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum Stmt {
    Return(Expr),
    Var(VarStmt),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Int32(i32),
    String(String),
    Ident(ExprIdent),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprIdent {
    Var(String),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Block,
    pub return_type: DataType,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub p_type: DataType,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VarStmt {
    pub data_type: DataType,
    pub name: String,
    pub expr: Expr,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int32(v) => write!(f, "{}", v),
            Expr::Ident(ident) => match ident {
                ExprIdent::Var(var) => write!(f, "{}", var.as_str()),
            },
            Expr::String(str) => write!(f, "{}", str.as_str()),
            Expr::None => write!(f, "None"),
        }
    }
}
