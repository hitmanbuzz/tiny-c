use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{
        AssignStmt, Ast, BinaryExpr, Block, Decl, Expr, FunctionDef, IdentExpr, IfBranch, IfStmt,
        Stmt, VarStmt,
    },
    token::{
        Token::{self},
        TokenData,
    },
    types::{DataType, Keyword},
};

pub struct Parser {
    pub ast: Ast,
    tokens: Peekable<IntoIter<TokenData>>,
}

struct ParseError {
    msg: String,
    line: usize,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenData>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            ast: Ast { decls: Vec::new() },
        }
    }

    pub fn parse(&mut self) {
        while let curr = self.next()
            && curr.token != Token::Eof
        {
            match self.parse_node(curr) {
                Ok(n) => self.ast.decls.push(n),
                Err(e) => eprintln!(
                    "[PARSER ERROR] [Line: {} | Pos: {}]: {}",
                    e.line, e.pos, e.msg
                ),
            }
        }
    }

    fn parse_node(&mut self, curr: TokenData) -> Result<Decl, ParseError> {
        let ident = match curr.token {
            Token::Identifier(str) => str,
            t => {
                return Err(ParseError {
                    msg: format!(
                        "expected `Identifier` at the start of program but found: {:?}",
                        t
                    ),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        let keyword = self.get_keyword(ident.as_str()).ok_or_else(|| ParseError {
            msg: format!("expected keyword but found: `{}`", ident),
            line: curr.line,
            pos: curr.pos,
        })?;

        let data_type = keyword.to_data_type().ok_or_else(|| ParseError {
            msg: format!(
                "expected keyword data-type but found keyword: {:?}",
                keyword
            ),
            line: curr.line,
            pos: curr.pos,
        })?;

        return self.parse_node_type(data_type);
    }

    fn parse_node_type(&mut self, data_type: DataType) -> Result<Decl, ParseError> {
        let mut curr = self.next();
        let name = match curr.token {
            Token::Identifier(i) => i,
            t => {
                return Err(ParseError {
                    msg: format!("expected node name(Identifier) but found: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        curr = self.next();
        match curr.token {
            Token::LeftParen => {
                let func = self.parse_func(data_type, name.as_str());
                match func {
                    Ok(f) => Ok(Decl::FuncDef(f)),
                    Err(err) => Err(err),
                }
            }
            Token::Equal => {
                let var = self.parse_var_stmt(data_type, name.as_str());
                match var {
                    Ok(v) => Ok(Decl::Var(v)),
                    Err(err) => Err(err),
                }
            }
            Token::SemiColon => Ok(Decl::Var(VarStmt {
                data_type,
                name,
                value: Expr::Empty,
                id: None,
                is_global: false,
            })),
            t => {
                return Err(ParseError {
                    msg: format!("invalid token after `Identifier ({})`: {:?}", name, t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        }
    }

    fn parse_var_stmt(&mut self, data_type: DataType, name: &str) -> Result<VarStmt, ParseError> {
        let expr = self.parse_expr(0.0)?;

        let curr = self.next();
        match curr.token {
            Token::SemiColon => {}
            t => {
                return Err(ParseError {
                    msg: format!(
                        "expected `SemiColon` at the end of var_stmt but found: {:?}",
                        t
                    ),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        }

        return Ok(VarStmt {
            data_type: data_type,
            name: name.to_string(),
            value: expr,
            id: None,
            is_global: false,
        });
    }

    fn parse_func(&mut self, data_type: DataType, name: &str) -> Result<FunctionDef, ParseError> {
        let mut curr = self.next();
        match curr.token {
            Token::RightParen => {}
            t => {
                return Err(ParseError {
                    msg: format!("expected `RightParen` but found: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        curr = self.next();
        match curr.token {
            Token::LeftCurlyBr => {}
            t => {
                return Err(ParseError {
                    msg: format!("expected `LeftCurlyBr` but found: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        let block = self.parse_block()?;
        curr = self.next();
        match curr.token {
            Token::RightCurlyBr => {}
            t => {
                return Err(ParseError {
                    msg: format!("expected `RightCurlyBr` but found: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        return Ok(FunctionDef {
            name: name.to_string(),
            params: Vec::new(),
            body: block,
            return_type: data_type,
        });
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let mut block = Block { stmts: Vec::new() };

        while self.peek().token != Token::RightCurlyBr && self.peek().token != Token::Eof {
            block.stmts.push(self.parse_stmt()?);
        }

        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let curr = self.next();
        let stmt = match curr.token {
            Token::Identifier(ident) => {
                let Some(keyword) = self.get_keyword(&ident) else {
                    return self.parse_assign_stmt(&ident);
                };

                match keyword {
                    Keyword::Int
                    | Keyword::Float
                    | Keyword::Bool
                    | Keyword::String
                    | Keyword::Void => {
                        // this is guarantee to work (hehehe)
                        let data_type = keyword.to_data_type().unwrap();
                        match self.parse_node_type(data_type)? {
                            Decl::FuncDef(fd) => Err(ParseError {
                                msg: format!("unexpected function within a function: {:?}", fd),
                                line: curr.line,
                                pos: curr.pos,
                            }),
                            Decl::Var(stmt) => Ok(Stmt::Var(stmt)),
                        }
                    }
                    Keyword::Return => self.parse_return_stmt(),
                    Keyword::If => self.parse_if_stmt(),
                    Keyword::Else => {
                        return Err(ParseError {
                            msg: format!("unexpected else stmt declration before a if stmt"),
                            line: curr.line,
                            pos: curr.pos,
                        });
                    }
                    k => {
                        return Err(ParseError {
                            msg: format!("invalid keyword found at parse_stmt: {:?}", k),
                            line: curr.line,
                            pos: curr.pos,
                        });
                    }
                }
            }
            t => {
                return Err(ParseError {
                    msg: format!("expected `Identifier` in parse_stmt but found: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };
        return stmt;
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        let mut stmt = IfStmt {
            branches: Vec::new(),
            else_stmt: None,
        };

        stmt.branches.push(self.parse_if_stmt_branch()?);

        loop {
            if !self.is_keyword("else") {
                break;
            }

            self.next();

            if self.is_keyword("if") {
                self.next();
                stmt.branches.push(self.parse_if_stmt_branch()?);
            } else {
                stmt.else_stmt = Some(self.parse_else_stmt()?);
                break;
            }
        }

        return Ok(Stmt::IfStmt(stmt));
    }

    fn parse_else_stmt(&mut self) -> Result<Block, ParseError> {
        let mut curr = self.next();
        if curr.token != Token::LeftCurlyBr {
            return Err(ParseError {
                msg: format!(
                    "expected `LeftCurlyBr` after else keyword but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        let body = self.parse_block()?;

        curr = self.next();
        if curr.token != Token::RightCurlyBr {
            return Err(ParseError {
                msg: format!(
                    "expected `RightCurlyBr` after the end of else body block but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        return Ok(Block { stmts: body.stmts });
    }

    fn parse_if_stmt_branch(&mut self) -> Result<IfBranch, ParseError> {
        let expr = self.parse_expr(0.0)?;

        let mut curr = self.next();
        if curr.token != Token::LeftCurlyBr {
            return Err(ParseError {
                msg: format!(
                    "expected `LeftCurlyBr` before the start of if_stmt block but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        let body = self.parse_block()?;

        curr = self.next();
        if curr.token != Token::RightCurlyBr {
            return Err(ParseError {
                msg: format!(
                    "expected `RightCurlyBr` at the end of if_stmt block but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        return Ok(IfBranch {
            cond_expr: expr,
            body,
        });
    }

    fn parse_assign_stmt(&mut self, target: &str) -> Result<Stmt, ParseError> {
        let mut curr = self.next();
        if curr.token != Token::Equal {
            return Err(ParseError {
                msg: format!(
                    "expected `Equal` in parse_assign_stmt but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        let expr = self.parse_expr(0.0)?;

        curr = self.next();
        if curr.token != Token::SemiColon {
            return Err(ParseError {
                msg: format!(
                    "expected `SemiColon` in parse_assign_stmt but found: {:?}",
                    curr.token
                ),
                line: curr.line,
                pos: curr.pos,
            });
        }

        return Ok(Stmt::Assign(AssignStmt {
            target: Expr::Ident(IdentExpr {
                name: target.to_string(),
                id: None,
                data_type: None,
            }),
            value: expr,
        }));
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.parse_expr(0.0);

        match expr {
            Ok(e) => {
                let curr = self.next();
                if curr.token != Token::SemiColon {
                    return Err(ParseError {
                        msg: format!("expected `SemiColon` but found: {:?}", curr),
                        line: curr.line,
                        pos: curr.pos,
                    });
                }
                Ok(Stmt::Return(e))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expr(&mut self, min_bp: f32) -> Result<Expr, ParseError> {
        let curr = self.next();
        let mut lhs = match curr.token {
            Token::String(str) => Expr::String(str),
            Token::Identifier(str) => match str.as_str() {
                "true" => Expr::Bool(true),
                "false" => Expr::Bool(false),
                _ => Expr::Ident(IdentExpr {
                    name: str,
                    id: None,
                    data_type: None,
                }),
            },
            Token::Number(str) => {
                if str.contains(".") {
                    let num = str.parse::<f32>().map_err(|_| ParseError {
                        msg: format!("failed to parse `{}` to f32", str),
                        line: curr.line,
                        pos: curr.pos,
                    })?;

                    Expr::Float32(num)
                } else {
                    let num = str.parse::<i32>().map_err(|_| ParseError {
                        msg: format!("failed to parse `{}` to i32", str),
                        line: curr.line,
                        pos: curr.pos,
                    })?;

                    Expr::Int32(num)
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expr(0.0)?;
                let curr = self.next();
                match curr.token {
                    Token::RightParen => expr,
                    t => {
                        return Err(ParseError {
                            msg: format!("expected `RightParen` but found: {:?}", t),
                            line: curr.line,
                            pos: curr.pos,
                        });
                    }
                }
            }
            t => {
                return Err(ParseError {
                    msg: format!("unexpected token in expr: {:?}", t),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        };

        loop {
            match self.peek().token {
                // FIX: EOF should not break always (it can return error)
                Token::SemiColon | Token::RightParen | Token::LeftCurlyBr | Token::Eof => break,
                Token::Plus
                | Token::Minus
                | Token::Star
                | Token::ForwardSlash
                | Token::Modulo
                | Token::And
                | Token::Or
                | Token::Less
                | Token::Greater
                | Token::LessEqual
                | Token::GreaterEqual
                | Token::EqualEqual
                | Token::NotEqual
                | Token::BitAnd
                | Token::BitOr
                | Token::BitXor
                | Token::BitLS
                | Token::BitRS => {
                    let (lbp, rbp, op) = self.peek().token.bin_op().ok_or_else(|| ParseError {
                        msg: format!("expected `Operator` but found: {:?}", self.peek().token),
                        line: self.peek().line,
                        pos: self.peek().pos,
                    })?;

                    if lbp < min_bp {
                        break;
                    }

                    self.next();
                    let rhs = self.parse_expr(rbp)?;
                    lhs = Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: lhs,
                        op: op,
                        right: rhs,
                    }));
                }
                t => {
                    return Err(ParseError {
                        msg: format!("unexpected token in expression: {:?}", t),
                        line: self.peek().line,
                        pos: self.peek().pos,
                    });
                }
            }
        }

        return Ok(lhs);
    }

    fn next(&mut self) -> TokenData {
        return self.tokens.next().unwrap_or(TokenData::default());
    }

    fn peek(&mut self) -> TokenData {
        return self.tokens.peek().unwrap_or(&TokenData::default()).clone();
    }

    fn is_keyword(&mut self, keyword: &str) -> bool {
        matches!(self.peek().token, Token::Identifier(name) if name == keyword)
    }

    /// get keyword type from ident string
    fn get_keyword(&self, ident: &str) -> Option<Keyword> {
        match ident {
            "int" => Some(Keyword::Int),
            "float" => Some(Keyword::Float),
            "void" => Some(Keyword::Void),
            "string" => Some(Keyword::String),
            "bool" => Some(Keyword::Bool),
            "return" => Some(Keyword::Return),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            _ => None,
        }
    }
}

// replace my hand written old stinking unit test cases with a AI one
// my unit cases was kinda fuckup if it failed and debugging was a hell
#[cfg(test)]
mod tests {
    use crate::{ast::BinaryOp, lexer::Lexer};

    use super::*;

    fn parse(source: &str) -> Ast {
        let mut lexer = Lexer::new(source);
        lexer.tokenize();
        let mut parser = Parser::new(lexer.tokens);
        parser.parse();
        parser.ast
    }

    fn assert_tokens(actual: &[TokenData], expected: &[Token]) {
        assert_eq!(
            actual.len(),
            expected.len(),
            "token count mismatch: got {}, expected {}",
            actual.len(),
            expected.len()
        );
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            assert_eq!(a.token, *e, "token mismatch at index {}", i);
        }
    }

    fn single_function(ast: &Ast) -> &FunctionDef {
        assert_eq!(ast.decls.len(), 1, "expected exactly one top-level decl");
        match &ast.decls[0] {
            Decl::FuncDef(f) => f,
            other => panic!("expected FuncDef, got {:?}", other),
        }
    }

    fn expect_int32(expr: &Expr, expected: i32) {
        match expr {
            Expr::Int32(n) => assert_eq!(*n, expected, "int32 literal mismatch"),
            other => panic!("expected Int32({}), got {:?}", expected, other),
        }
    }

    fn expect_ident(expr: &Expr, expected_name: &str) {
        match expr {
            Expr::Ident(id) => {
                assert_eq!(id.name, expected_name, "identifier name mismatch");
            }
            other => panic!("expected Ident({}), got {:?}", expected_name, other),
        }
    }

    fn expect_binary(expr: &Expr, expected_op: BinaryOp) -> &BinaryExpr {
        match expr {
            Expr::BinaryExpr(b) => {
                assert_eq!(b.op, expected_op, "binary op mismatch");
                b
            }
            other => panic!("expected BinaryExpr({:?}), got {:?}", expected_op, other),
        }
    }

    #[test]
    fn test_return_stmt() {
        let source = "
            int main() {
                return 69;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        let good_tokens = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("return".to_string()),
            Token::Number("69".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];
        assert_tokens(&lexer.tokens, &good_tokens);

        let ast = parse(source);

        let func = single_function(&ast);
        assert_eq!(func.name, "main");
        assert_eq!(func.return_type, DataType::Int32);
        assert!(func.params.is_empty(), "main should have no params");

        assert_eq!(func.body.stmts.len(), 1, "body should have exactly 1 stmt");

        match &func.body.stmts[0] {
            Stmt::Return(expr) => expect_int32(expr, 69),
            other => panic!("expected Return stmt, got {:?}", other),
        }
    }

    #[test]
    fn test_var_stmt() {
        let source = "
            int main() {
                int a = 67;
                return 69;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        let good_tokens = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("int".to_string()),
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Number("67".to_string()),
            Token::SemiColon,
            Token::Identifier("return".to_string()),
            Token::Number("69".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];
        assert_tokens(&lexer.tokens, &good_tokens);

        let ast = parse(source);

        let func = single_function(&ast);
        assert_eq!(func.name, "main");
        assert_eq!(func.return_type, DataType::Int32);
        assert_eq!(func.body.stmts.len(), 2);

        match &func.body.stmts[0] {
            Stmt::Var(v) => {
                assert_eq!(v.data_type, DataType::Int32);
                assert_eq!(v.name, "a");
                expect_int32(&v.value, 67);
                assert!(v.id.is_none(), "parser should not assign an id");
                assert!(!v.is_global, "local var should not be global");
            }
            other => panic!("expected Var stmt, got {:?}", other),
        }

        match &func.body.stmts[1] {
            Stmt::Return(expr) => expect_int32(expr, 69),
            other => panic!("expected Return stmt, got {:?}", other),
        }
    }

    #[test]
    fn test_assign_stmt() {
        let source = "
            int main() {
                int x = 69;
                x = 67;
                return x;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        let good_tokens = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("int".to_string()),
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number("69".to_string()),
            Token::SemiColon,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number("67".to_string()),
            Token::SemiColon,
            Token::Identifier("return".to_string()),
            Token::Identifier("x".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];
        assert_tokens(&lexer.tokens, &good_tokens);

        let ast = parse(source);

        let func = single_function(&ast);
        assert_eq!(func.body.stmts.len(), 3);

        match &func.body.stmts[0] {
            Stmt::Var(v) => {
                assert_eq!(v.data_type, DataType::Int32);
                assert_eq!(v.name, "x");
                expect_int32(&v.value, 69);
                assert!(v.id.is_none());
                assert!(!v.is_global);
            }
            other => panic!("expected Var stmt, got {:?}", other),
        }

        match &func.body.stmts[1] {
            Stmt::Assign(a) => {
                expect_ident(&a.target, "x");
                expect_int32(&a.value, 67);
            }
            other => panic!("expected Assign stmt, got {:?}", other),
        }

        match &func.body.stmts[2] {
            Stmt::Return(expr) => expect_ident(expr, "x"),
            other => panic!("expected Return stmt, got {:?}", other),
        }
    }

    #[test]
    fn test_binary_expr() {
        let source = "
            int main() {
                int a = 1 * (2 + 3);
                int b = 1 + 2 * 3 * 4 + 5 / 6 - 7;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        let good_tokens = vec![
            Token::Identifier("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBr,
            Token::Identifier("int".to_string()),
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Number("1".to_string()),
            Token::Star,
            Token::LeftParen,
            Token::Number("2".to_string()),
            Token::Plus,
            Token::Number("3".to_string()),
            Token::RightParen,
            Token::SemiColon,
            Token::Identifier("int".to_string()),
            Token::Identifier("b".to_string()),
            Token::Equal,
            Token::Number("1".to_string()),
            Token::Plus,
            Token::Number("2".to_string()),
            Token::Star,
            Token::Number("3".to_string()),
            Token::Star,
            Token::Number("4".to_string()),
            Token::Plus,
            Token::Number("5".to_string()),
            Token::ForwardSlash,
            Token::Number("6".to_string()),
            Token::Minus,
            Token::Number("7".to_string()),
            Token::SemiColon,
            Token::RightCurlyBr,
            Token::Eof,
        ];

        assert_tokens(&lexer.tokens, &good_tokens);

        let ast = parse(source);
        let func = single_function(&ast);
        assert_eq!(func.body.stmts.len(), 2);

        let a_value = match &func.body.stmts[0] {
            Stmt::Var(v) => {
                assert_eq!(v.data_type, DataType::Int32);
                assert_eq!(v.name, "a");
                &v.value
            }
            other => panic!("expected Var stmt, got {:?}", other),
        };

        // Expected tree:
        //        *
        //       / \
        //      1   +
        //         / \
        //        2   3
        let mul = expect_binary(a_value, BinaryOp::Mul);
        expect_int32(&mul.left, 1);
        let add = expect_binary(&mul.right, BinaryOp::Add);
        expect_int32(&add.left, 2);
        expect_int32(&add.right, 3);

        let b_value = match &func.body.stmts[1] {
            Stmt::Var(v) => {
                assert_eq!(v.data_type, DataType::Int32);
                assert_eq!(v.name, "b");
                &v.value
            }
            other => panic!("expected Var stmt, got {:?}", other),
        };

        // Expected tree:
        //                 -
        //                / \
        //               +   7
        //              / \
        //             +   /
        //            / \ / \
        //           1  * 5  6
        //             / \
        //            *   4
        //           / \
        //          2   3

        let sub = expect_binary(b_value, BinaryOp::Sub);
        expect_int32(&sub.right, 7);

        let outer_add = expect_binary(&sub.left, BinaryOp::Add);
        let div = expect_binary(&outer_add.right, BinaryOp::Div);
        expect_int32(&div.left, 5);
        expect_int32(&div.right, 6);

        let inner_add = expect_binary(&outer_add.left, BinaryOp::Add);
        expect_int32(&inner_add.left, 1);
        let mul_outer = expect_binary(&inner_add.right, BinaryOp::Mul);
        expect_int32(&mul_outer.right, 4);

        let mul_inner = expect_binary(&mul_outer.left, BinaryOp::Mul);
        expect_int32(&mul_inner.left, 2);
        expect_int32(&mul_inner.right, 3);
    }
}
