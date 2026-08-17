use std::fmt::Display;

use crate::types::DataType;

pub struct Ast {
    pub f: Option<FunctionDef>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Stmt {
    RETURN(Expr),
    VARIABLE(DataType, String, Expr),
}

#[derive(Debug)]
pub enum Expr {
    INT32(i32),
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

pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::INT32(v) => write!(f, "{}", v),
        }
    }
}
