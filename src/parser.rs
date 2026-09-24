use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{AssignStmt, Ast, BinaryExpr, Block, Decl, Expr, FunctionDef, IdentExpr, Stmt, VarStmt},
    token::{Token, TokenData},
    types::{DataType, IdentType, Keyword},
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

        let ident_type = self.get_ident(&ident).ok_or_else(|| ParseError {
            msg: format!(
                "conversion of `Identifier ({})` to its distinct type is not implemented",
                ident,
            ),
            line: curr.line,
            pos: curr.pos,
        })?;

        match ident_type {
            IdentType::DataType(data_type) => self.parse_node_type(data_type),
            IdentType::Keyword(keyword) => {
                return Err(ParseError {
                    msg: format!("expected `DataType` but found: `Keyword ({:?})`", keyword),
                    line: curr.line,
                    pos: curr.pos,
                });
            }
        }
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
            Token::Identifier(str) => {
                if let Some(ident_type) = self.get_ident(&str) {
                    match ident_type {
                        IdentType::DataType(data_type) => {
                            let node = self.parse_node_type(data_type)?;
                            match node {
                                Decl::FuncDef(f) => Err(ParseError {
                                    msg: format!("unexpected function within a function: {:?}", f),
                                    line: curr.line,
                                    pos: curr.pos,
                                }),
                                Decl::Var(v) => Ok(Stmt::Var(v)),
                            }
                        }
                        IdentType::Keyword(keyword) => match keyword {
                            Keyword::Return => self.parse_return_stmt(),
                        },
                    }
                } else {
                    return self.parse_assign_stmt(&str);
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
        if self.peek().token == Token::SemiColon {
            return Ok(Expr::Empty);
        }

        let curr = self.next();
        let mut lhs = match curr.token {
            Token::String(str) => Expr::String(str),
            Token::Identifier(str) => Expr::Ident(IdentExpr {
                name: str,
                id: None,
                data_type: None,
            }),
            Token::Number(str) => {
                let num = str.parse::<i32>().map_err(|_| ParseError {
                    msg: format!("failed to parse `{}` to i32", str),
                    line: curr.line,
                    pos: curr.pos,
                })?;

                Expr::Int32(num)
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
                Token::SemiColon | Token::RightParen | Token::Eof => break,
                Token::Plus | Token::Minus | Token::Star | Token::ForwardSlash | Token::Modulo => {
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

    fn get_ident(&self, ident: &str) -> Option<IdentType> {
        match ident {
            "int" => Some(IdentType::DataType(DataType::Int)),
            "void" => Some(IdentType::DataType(DataType::Void)),
            "char*" => Some(IdentType::DataType(DataType::CharPtr)),
            "return" => Some(IdentType::Keyword(Keyword::Return)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::BinaryOp, lexer::Lexer};

    use super::*;

    #[test]
    fn test_return_stmt() {
        let source = "
            int main() {
                return 69;
            }
        ";
        let mut lexer = Lexer::new(source);
        lexer.tokenize();

        assert!(
            lexer.tokens.len() > 0,
            "should have 10 tokens but got 0 instead"
        );

        let good_tokens: Vec<Token> = vec![
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

        assert_eq!(
            lexer.tokens.len(),
            good_tokens.len(),
            "should have same 10 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i].token, good_tokens[i]);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            decls: vec![Decl::FuncDef(FunctionDef {
                name: String::from("main"),
                params: vec![],
                body: Block {
                    stmts: vec![Stmt::Return(Expr::Int32(69))],
                },
                return_type: DataType::Int,
            })],
        };

        assert_eq!(parser.ast, good_ast);
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

        assert!(
            lexer.tokens.len() > 0,
            "should have 10 tokens but got 0 instead"
        );

        let good_tokens: Vec<Token> = vec![
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

        assert_eq!(
            good_tokens.len(),
            lexer.tokens.len(),
            "should have same 15 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(good_tokens[i], lexer.tokens[i].token);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            decls: vec![Decl::FuncDef(FunctionDef {
                name: String::from("main"),
                params: vec![],
                body: Block {
                    stmts: vec![
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: String::from("a"),
                            value: Expr::Int32(67),
                            id: None,
                            is_global: false,
                        }),
                        Stmt::Return(Expr::Int32(69)),
                    ],
                },
                return_type: DataType::Int,
            })],
        };

        assert_eq!(parser.ast, good_ast);
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

        assert!(
            lexer.tokens.len() > 0,
            "should have 19 tokens but got 0 instead"
        );

        let good_tokens: Vec<Token> = vec![
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

        assert_eq!(
            good_tokens.len(),
            lexer.tokens.len(),
            "should have same 19 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(good_tokens[i], lexer.tokens[i].token);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            decls: vec![Decl::FuncDef(FunctionDef {
                name: String::from("main"),
                params: vec![],
                body: Block {
                    stmts: vec![
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: String::from("x"),
                            value: Expr::Int32(69),
                            id: None,
                            is_global: false,
                        }),
                        Stmt::Assign(AssignStmt {
                            target: Expr::Ident(IdentExpr {
                                name: "x".to_string(),
                                id: None,
                                data_type: None,
                            }),
                            value: Expr::Int32(67),
                        }),
                        Stmt::Return(Expr::Ident(IdentExpr {
                            name: "x".to_string(),
                            id: None,
                            data_type: None,
                        })),
                    ],
                },
                return_type: DataType::Int,
            })],
        };

        assert_eq!(parser.ast, good_ast);
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

        let good_tokens: Vec<Token> = vec![
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

        assert_eq!(
            good_tokens.len(),
            lexer.tokens.len(),
            "should have same 35 tokens"
        );

        for i in 0..lexer.tokens.len() {
            assert_eq!(good_tokens[i], lexer.tokens[i].token);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            decls: vec![Decl::FuncDef(FunctionDef {
                name: "main".to_string(),
                params: vec![],
                body: Block {
                    stmts: vec![
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: "a".to_string(),

                            // 1 * (2 + 3)
                            value: Expr::BinaryExpr(Box::new(BinaryExpr {
                                left: Expr::Int32(1),
                                op: BinaryOp::Mul,
                                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                                    left: Expr::Int32(2),
                                    op: BinaryOp::Add,
                                    right: Expr::Int32(3),
                                })),
                            })),
                            id: None,
                            is_global: false,
                        }),
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: "b".to_string(),
                            value: Expr::BinaryExpr(Box::new(BinaryExpr {
                                left: Expr::BinaryExpr(Box::new(BinaryExpr {
                                    left: Expr::BinaryExpr(Box::new(BinaryExpr {
                                        left: Expr::Int32(1),
                                        op: BinaryOp::Add,
                                        right: Expr::BinaryExpr(Box::new(BinaryExpr {
                                            left: Expr::BinaryExpr(Box::new(BinaryExpr {
                                                left: Expr::Int32(2),
                                                op: BinaryOp::Mul,
                                                right: Expr::Int32(3),
                                            })),
                                            op: BinaryOp::Mul,
                                            right: Expr::Int32(4),
                                        })),
                                    })),
                                    op: BinaryOp::Add,
                                    right: Expr::BinaryExpr(Box::new(BinaryExpr {
                                        left: Expr::Int32(5),
                                        op: BinaryOp::Div,
                                        right: Expr::Int32(6),
                                    })),
                                })),
                                op: BinaryOp::Sub,
                                right: Expr::Int32(7),
                            })),
                            id: None,
                            is_global: false,
                        }),
                    ],
                },
                return_type: DataType::Int,
            })],
        };

        assert_eq!(parser.ast, good_ast);
    }
}
