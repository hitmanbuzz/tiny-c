use std::fmt::{self, Display, Formatter};

use crate::types::DataType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    pub decls: Vec<Decl>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Decl {
    FuncDef(FunctionDef),
    Var(VarStmt),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    Return(Expr),
    Var(VarStmt),
    Assign(AssignStmt),
    IfStmt(IfStmt),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdentExpr {
    pub name: String,
    pub id: Option<usize>,
    pub data_type: Option<DataType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Int32(i32),
    String(String),
    Ident(IdentExpr),
    BinaryExpr(Box<BinaryExpr>),
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDef {
    pub return_type: DataType,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub name: String,
    pub p_type: DataType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VarStmt {
    pub data_type: DataType,
    pub name: String,
    pub value: Expr,
    pub id: Option<usize>,
    pub is_global: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignStmt {
    pub target: Expr,
    pub value: Expr,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,

    Less,
    Greater,

    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmt {
    pub branches: Vec<IfBranch>,
    pub else_stmt: Option<Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfBranch {
    pub cond_expr: Expr,
    pub body: Block,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: BinaryOp,
    pub right: Expr,
}

// i use AI to make a good looking AST Display Trait implementation
// my previous own implementation was kinda bad

impl Display for Ast {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "AST")?;

        for (i, decl) in self.decls.iter().enumerate() {
            let last = i == self.decls.len() - 1;
            fmt_decl(f, decl, "", last)?;
        }

        Ok(())
    }
}

fn fmt_decl(f: &mut Formatter<'_>, decl: &Decl, prefix: &str, last: bool) -> fmt::Result {
    let branch = if last { "└── " } else { "├── " };

    match decl {
        Decl::FuncDef(func) => {
            writeln!(f, "{prefix}{branch}FunctionDef")?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            writeln!(f, "{child_prefix}├── ReturnType: {:?}", func.return_type)?;
            writeln!(f, "{child_prefix}├── Name: {}", func.name)?;

            fmt_params(f, &func.params, &child_prefix, func.body.stmts.is_empty())?;

            let body_prefix = child_prefix;

            writeln!(f, "{body_prefix}└── Body")?;

            let stmt_prefix = format!("{body_prefix}    ");

            for (i, stmt) in func.body.stmts.iter().enumerate() {
                let last_stmt = i == func.body.stmts.len() - 1;
                fmt_stmt(f, stmt, &stmt_prefix, last_stmt)?;
            }
        }

        Decl::Var(var) => {
            writeln!(f, "{prefix}{branch}VarDecl")?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            writeln!(f, "{child_prefix}├── Type: {:?}", var.data_type)?;
            writeln!(f, "{child_prefix}├── Name: {}", var.name)?;
            writeln!(f, "{child_prefix}├── ID: {:?}", var.id)?;
            writeln!(f, "{child_prefix}└── IsGlobal: {}", var.is_global)?;
            writeln!(f, "{child_prefix}└── Value")?;

            let expr_prefix = format!("{child_prefix}    ");
            fmt_expr(f, &var.value, &expr_prefix, true)?;
        }
    }

    Ok(())
}

fn fmt_params(f: &mut Formatter<'_>, params: &[Param], prefix: &str, last: bool) -> fmt::Result {
    if params.is_empty() {
        writeln!(f, "{prefix}├── Parameters: []")?;
        return Ok(());
    }

    writeln!(f, "{prefix}├── Parameters")?;

    let param_prefix = format!("{prefix}│   ");

    for (i, param) in params.iter().enumerate() {
        let last_param = i == params.len() - 1;

        let branch = if last_param {
            "└── "
        } else {
            "├── "
        };

        writeln!(
            f,
            "{param_prefix}{branch}Param: {} ({:?})",
            param.name, param.p_type
        )?;
    }

    let _ = last;
    Ok(())
}

fn fmt_stmt(f: &mut Formatter<'_>, stmt: &Stmt, prefix: &str, last: bool) -> fmt::Result {
    let branch = if last { "└── " } else { "├── " };

    match stmt {
        Stmt::Return(expr) => {
            writeln!(f, "{prefix}{branch}Return")?;

            let expr_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            fmt_expr(f, expr, &expr_prefix, true)?;
        }

        Stmt::Var(var) => {
            writeln!(f, "{prefix}{branch}VarDecl")?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            writeln!(f, "{child_prefix}├── Type: {:?}", var.data_type)?;
            writeln!(f, "{child_prefix}├── Name: {}", var.name)?;
            writeln!(f, "{child_prefix}├── ID: {:?}", var.id)?;
            writeln!(f, "{child_prefix}└── IsGlobal: {}", var.is_global)?;
            writeln!(f, "{child_prefix}└── Value")?;

            let expr_prefix = format!("{child_prefix}    ");
            fmt_expr(f, &var.value, &expr_prefix, true)?;
        }
        Stmt::Assign(assign) => {
            writeln!(f, "{prefix}{branch}Assign")?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            writeln!(f, "{child_prefix}├── Target")?;

            // Target is always followed by Value, so it's never the "last" child.
            let target_prefix = format!("{child_prefix}│   ");
            fmt_expr(f, &assign.target, &target_prefix, true)?;

            writeln!(f, "{child_prefix}└── Value")?;

            let value_prefix = format!("{child_prefix}    ");
            fmt_expr(f, &assign.value, &value_prefix, true)?;
        }
        Stmt::IfStmt(if_stmt) => {
            writeln!(f, "{prefix}{branch}IfStmt")?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            for (i, if_branch) in if_stmt.branches.iter().enumerate() {
                let is_last_branch = i == if_stmt.branches.len() - 1 && if_stmt.else_stmt.is_none();

                let branch_connector = if is_last_branch {
                    "└── "
                } else {
                    "├── "
                };

                let branch_name = if i == 0 { "Branch" } else { "ElseIf" };

                writeln!(f, "{child_prefix}{branch_connector}{branch_name}")?;

                let branch_prefix = format!(
                    "{child_prefix}{}",
                    if is_last_branch { "    " } else { "│   " }
                );

                // condition
                writeln!(f, "{branch_prefix}├── Condition")?;

                let condition_prefix = format!("{branch_prefix}│   ");

                fmt_expr(f, &if_branch.cond_expr, &condition_prefix, true)?;

                // body
                writeln!(f, "{branch_prefix}└── Body")?;

                let body_prefix = format!("{branch_prefix}    ");

                for (j, stmt) in if_branch.body.stmts.iter().enumerate() {
                    let last_stmt = j == if_branch.body.stmts.len() - 1;

                    fmt_stmt(f, stmt, &body_prefix, last_stmt)?;
                }
            }

            // else
            if let Some(else_block) = &if_stmt.else_stmt {
                writeln!(f, "{child_prefix}└── Else")?;

                let else_prefix = format!("{child_prefix}    ");

                writeln!(f, "{else_prefix}└── Body")?;

                let body_prefix = format!("{else_prefix}    ");

                for (i, stmt) in else_block.stmts.iter().enumerate() {
                    let last_stmt = i == else_block.stmts.len() - 1;

                    fmt_stmt(f, stmt, &body_prefix, last_stmt)?;
                }
            }
        }
    }

    Ok(())
}

fn fmt_expr(f: &mut Formatter<'_>, expr: &Expr, prefix: &str, last: bool) -> fmt::Result {
    let branch = if last { "└── " } else { "├── " };

    match expr {
        Expr::Int32(value) => {
            writeln!(f, "{prefix}{branch}Int32: {value}")?;
        }

        Expr::String(value) => {
            writeln!(f, "{prefix}{branch}String: {:?}", value)?;
        }

        Expr::Ident(expr) => {
            writeln!(
                f,
                "{prefix}{branch}Ident: {}({:?}) - {:?}",
                expr.name, expr.id, expr.data_type
            )?;
        }

        Expr::Empty => {
            writeln!(f, "{prefix}{branch}Empty")?;
        }

        Expr::BinaryExpr(binary) => {
            writeln!(f, "{prefix}{branch}BinaryExpr: {:?}", binary.op)?;

            let child_prefix = format!("{prefix}{}", if last { "    " } else { "│   " });

            fmt_expr(f, &binary.left, &child_prefix, false)?;
            fmt_expr(f, &binary.right, &child_prefix, true)?;
        }
    }

    Ok(())
}
