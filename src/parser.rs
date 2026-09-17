use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::{Ast, BinaryExpr, Block, Expr, FunctionDef, Node, Stmt, VarStmt},
    token::Token,
    types::{DataType, IDENTIFIERS, IdentType, Keyword},
};

pub struct Parser {
    pub ast: Ast,
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            ast: Ast { nodes: Vec::new() },
        }
    }

    pub fn parse(&mut self) {
        while let Some(token) = self.tokens.next()
            && token != Token::Eof
        {
            match self.parse_node(token) {
                Ok(n) => self.ast.nodes.push(n),
                Err(e) => eprintln!("{}", e),
            }
        }
    }

    fn parse_node(&mut self, curr: Token) -> Result<Node, String> {
        let ident = match curr {
            Token::Identifier(str) => str,
            t => {
                return Err(format!(
                    "expected `Identifier` at the start of program but found: {:?}",
                    t
                ));
            }
        };

        let ident_type = self.get_ident(&ident).ok_or_else(|| {
            format!(
                "conversion of `Identifier ({})` to its distinct type is not implemented",
                ident,
            )
        })?;

        match *ident_type {
            IdentType::DataType(data_type) => self.parse_node_type(data_type),
            IdentType::Keyword(keyword) => {
                return Err(format!(
                    "expected `DataType` but found: `Keyword ({:?})`",
                    keyword
                ));
            }
        }
    }

    fn parse_node_type(&mut self, data_type: DataType) -> Result<Node, String> {
        let name = match self.tokens.next().unwrap_or(Token::Eof) {
            Token::Identifier(i) => i,
            t => {
                return Err(format!("expected node name(Identifier) but found: {:?}", t));
            }
        };

        match self.tokens.next().unwrap_or(Token::Eof) {
            Token::LeftParen => {
                let func = self.parse_func(data_type, name.as_str());
                match func {
                    Ok(f) => Ok(Node::FuncDef(f)),
                    Err(err) => Err(err),
                }
            }
            Token::Equal => {
                let var = self.parse_var_stmt(data_type, name.as_str());
                match var {
                    Ok(v) => Ok(Node::Var(v)),
                    Err(err) => Err(err),
                }
            }
            t => {
                return Err(format!(
                    "invalid token after `Identifier ({})`: {:?}",
                    name, t
                ));
            }
        }
    }

    fn parse_var_stmt(&mut self, data_type: DataType, name: &str) -> Result<VarStmt, String> {
        let expr = self.parse_expr(0.0)?;

        match self.tokens.next().unwrap_or(Token::Eof) {
            Token::SemiColon => {}
            t => {
                return Err(format!(
                    "expected `SemiColon` at the end of var_stmt but found: {:?}",
                    t
                ));
            }
        }

        return Ok(VarStmt {
            data_type: data_type,
            name: name.to_string(),
            expr: expr,
        });
    }

    fn parse_func(&mut self, data_type: DataType, name: &str) -> Result<FunctionDef, String> {
        match self.tokens.next().unwrap_or(Token::Eof) {
            Token::RightParen => {}
            t => return Err(format!("expected `RightParen` but found: {:?}", t)),
        };

        match self.tokens.next().unwrap_or(Token::Eof) {
            Token::LeftCurlyBr => {}
            t => return Err(format!("expected `LeftCurlyBr` but found: {:?}", t)),
        };

        let block = self.parse_block()?;

        match self.tokens.next().unwrap_or(Token::Eof) {
            Token::RightCurlyBr => {}
            t => return Err(format!("expected `RightCurlyBr` but found: {:?}", t)),
        };

        return Ok(FunctionDef {
            name: name.to_string(),
            params: Vec::new(),
            body: block,
            return_type: data_type,
        });
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut block = Block { stmts: Vec::new() };

        while let Some(t) = self.tokens.peek() {
            match t {
                // FIX: EOF should not be a good signal (return Err)
                Token::RightCurlyBr | Token::Eof => break,
                _ => block.stmts.push(self.parse_stmt()?),
            }
        }

        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        let ident_type = match self.tokens.next().unwrap_or(Token::Eof) {
            Token::Identifier(str) => self.get_ident(&str).ok_or_else(|| {
                format!(
                    "conversion of `Identifier ({})` to its distinct type is not implemented",
                    str
                )
            })?,
            t => return Err(format!("expected `Identifier` but found: {:?}", t)),
        };

        match *ident_type {
            IdentType::DataType(data_type) => {
                let node = self.parse_node_type(data_type)?;
                match node {
                    Node::FuncDef(f) => {
                        Err(format!("unexpected function within a function: {:?}", f))
                    }
                    Node::Var(v) => Ok(Stmt::Var(v)),
                }
            }
            IdentType::Keyword(keyword) => match keyword {
                Keyword::Return => Ok(self.parse_return_stmt()?),
            },
        }
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr(0.0);

        match expr {
            Ok(e) => {
                let curr = self.tokens.next().unwrap_or(Token::Eof);
                if curr != Token::SemiColon {
                    return Err(format!("expected `SemiColon` but found: {:?}", curr));
                }
                Ok(Stmt::Return(e))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expr(&mut self, min_bp: f32) -> Result<Expr, String> {
        let mut lhs = match self.tokens.next().unwrap_or(Token::Eof) {
            Token::String(str) => Expr::String(str),
            Token::Identifier(str) => Expr::Ident(str),
            Token::Number(str) => {
                let num = str
                    .parse::<i32>()
                    .map_err(|_| format!("failed to parse `{}` to i32", str))?;
                Expr::Int32(num)
            }
            Token::LeftParen => {
                let expr = self.parse_expr(0.0)?;
                match self.tokens.next().unwrap_or(Token::Eof) {
                    Token::RightParen => expr,
                    t => return Err(format!("expected `RightParen` but found: {:?}", t)),
                }
            }
            t => return Err(format!("unexpected token in expr: {:?}", t)),
        };

        loop {
            match self.tokens.peek().unwrap_or(&Token::Eof) {
                // FIX: EOF should not break always (it can return error)
                Token::SemiColon | Token::RightParen | Token::Eof => break,
                Token::Plus | Token::Minus | Token::Star | Token::ForwardSlash | Token::Modulo => {
                    let curr = self.tokens.peek().unwrap(); // this should not panic
                    let (lbp, rbp, op) = curr
                        .bin_op()
                        .ok_or_else(|| format!("expected `Operator` but found: {:?}", curr))?;

                    if lbp < min_bp {
                        break;
                    }

                    self.tokens.next();
                    let rhs = self.parse_expr(rbp)?;
                    lhs = Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: lhs,
                        op,
                        right: rhs,
                    }));
                }
                t => return Err(format!("unexpected token in expression: {:?}", t)),
            }
        }

        return Ok(lhs);
    }

    fn get_ident(&self, ident: &str) -> Option<&IdentType> {
        IDENTIFIERS.get(ident)
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

        assert_eq!(lexer.errors.len(), 0);

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            nodes: vec![Node::FuncDef(FunctionDef {
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
            lexer.tokens.len(),
            good_tokens.len(),
            "should have same 15 tokens"
        );

        assert_eq!(lexer.errors.len(), 0);

        for i in 0..lexer.tokens.len() {
            assert_eq!(lexer.tokens[i], good_tokens[i]);
        }

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            nodes: vec![Node::FuncDef(FunctionDef {
                name: String::from("main"),
                params: vec![],
                body: Block {
                    stmts: vec![
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: String::from("a"),
                            expr: Expr::Int32(67),
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

        assert_eq!(lexer.tokens, good_tokens);
        assert!(lexer.errors.is_empty());

        let mut parser = Parser::new(lexer.tokens);
        parser.parse();

        let good_ast = Ast {
            nodes: vec![Node::FuncDef(FunctionDef {
                name: "main".to_string(),
                params: vec![],
                body: Block {
                    stmts: vec![
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: "a".to_string(),

                            // 1 * (2 + 3)
                            expr: Expr::BinaryExpr(Box::new(BinaryExpr {
                                left: Expr::Int32(1),
                                op: BinaryOp::Mul,
                                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                                    left: Expr::Int32(2),
                                    op: BinaryOp::Add,
                                    right: Expr::Int32(3),
                                })),
                            })),
                        }),
                        Stmt::Var(VarStmt {
                            data_type: DataType::Int,
                            name: "b".to_string(),

                            // ((1 + ((2 * 3) * 4)) + (5 / 6)) - 7
                            expr: Expr::BinaryExpr(Box::new(BinaryExpr {
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
                        }),
                    ],
                },
                return_type: DataType::Int,
            })],
        };

        assert_eq!(parser.ast, good_ast);
    }
}
