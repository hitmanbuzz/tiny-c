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
    Ident(String),
    BinaryExpr(Box<BinaryExpr>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDef {
    pub return_type: DataType,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Block,
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

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: BinaryOp,
    pub right: Expr,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::FuncDef(func) => {
                writeln!(
                    f,
                    "func_return_type ({:?})  func_name ({})",
                    func.return_type, func.name
                )?;
                for stmt in func.body.stmts.iter() {
                    writeln!(f, "    {}", stmt)?;
                }
                Ok(())
            }
            Node::Var(var_stmt) => write!(
                f,
                "var_type ({:?})  var_name ({})  =  var_expr ({:#?})",
                var_stmt.data_type, var_stmt.name, var_stmt.expr
            ),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Return(expr) => write!(f, "func_return_stmt ({:?})", expr),
            Stmt::Var(var_stmt) => write!(
                f,
                "var_type ({:?})  var_name ({})  =  var_expr ({:?})",
                var_stmt.data_type, var_stmt.name, var_stmt.expr
            ),
        }
    }
}

// impl Display for Expr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Expr::Int32(v) => write!(f, "Int32({})", v),
//             Expr::Ident(ident) => match ident {
//                 ExprIdent::Ident(ident) => write!(f, "Ident({})", ident),
//             },
//             Expr::String(str) => write!(f, "String({})", str.as_str()),
//             Expr::BinaryExpr(expr) => {
//                 write!(f, "BinaryExpr({} {} {})", expr.left, expr.op, expr.right)
//             }
//         }
//     }
// }
