use crate::{
    ast::{Ast, BinaryExpr, Block, Expr, FunctionDef, Node, Stmt, VarStmt},
    token::Token,
    types::{DataType, IDENTIFIERS, IdentType, Keyword},
};

pub struct Parser {
    pub ast: Ast,
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            idx: 0,
            ast: Ast {
                nodes: Vec::new(),
                err: None,
            },
        }
    }

    pub fn parse(&mut self) {
        while self.peek() != Token::Eof {
            let curr = self.next();
            match curr {
                Token::Identifier(ident) => {
                    let node = self.parse_node(ident.as_str());
                    match node {
                        Ok(n) => self.ast.nodes.push(n),
                        Err(err) => {
                            self.ast.err = Some(err);
                            return;
                        }
                    }
                }
                _ => {
                    let err_msg = format!(
                        "expected `IDENTIFIER` at the start of program but found: `{:?}`",
                        curr
                    );
                    self.ast.err = Some(err_msg);
                    return;
                }
            }
        }
    }

    fn parse_node(&mut self, ident: &str) -> Result<Node, String> {
        let ident_type = self.get_ident(ident).ok_or_else(|| {
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
        let mut curr = self.next();

        if let Token::Identifier(name) = curr {
            if self.get_ident(name.as_str()).is_some() {
                return Err(format!(
                    "conversion of `Identifier ({})` to its distinct type is not implemented",
                    name,
                ));
            }

            curr = self.next();
            match curr {
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
                _ => {
                    return Err(format!(
                        "invalid token after `Identifier ({})`: {:?}",
                        name, curr
                    ));
                }
            }
        } else {
            return Err(format!(
                "expected `Identifier (name)` but found: {:?}",
                curr
            ));
        }
    }

    fn parse_var_stmt(&mut self, data_type: DataType, name: &str) -> Result<VarStmt, String> {
        let expr = self.parse_expr(0.0)?;

        let curr = self.next();
        if curr != Token::SemiColon {
            return Err(format!(
                "expected `SemiColon` at the end of var_stmt but found: {:?}",
                curr
            ));
        }

        return Ok(VarStmt {
            data_type: data_type,
            name: name.to_string(),
            expr: expr,
        });
    }

    fn parse_func(&mut self, data_type: DataType, name: &str) -> Result<FunctionDef, String> {
        let mut curr = self.next();
        if curr != Token::RightParen {
            return Err(format!("expected `RightParen` but found: {:?}", curr));
        }

        curr = self.next();
        if curr != Token::LeftCurlyBr {
            return Err(format!("expected `LeftCurlyBr` but found: {:?}", curr));
        }

        let block = self.parse_block()?;

        curr = self.next();
        if curr != Token::RightCurlyBr {
            return Err(format!("expected `RightCurlyBr` but found: {:?}", curr));
        }

        return Ok(FunctionDef {
            name: name.to_string(),
            params: Vec::new(),
            body: block,
            return_type: data_type,
        });
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut block = Block { stmts: Vec::new() };

        while self.peek() != Token::RightCurlyBr {
            let stmt = self.parse_stmt()?;
            block.stmts.push(stmt);
        }

        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if !matches!(self.peek(), Token::Identifier(_)) {
            return Err(format!(
                "expected `Identifier` on parse_stmt but found {:?}",
                self.peek()
            ));
        }

        // we are sure this is ident
        let ident = self.next().ident_name();
        let ident_type = self.get_ident(ident.as_str()).ok_or_else(|| {
            format!(
                "conversion of `Identifier ({})` to its distinct type is not implemented",
                ident
            )
        })?;

        match ident_type {
            IdentType::DataType(data_type) => {
                let node = self.parse_node_type(*data_type)?;
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
                let curr = self.next();
                if curr != Token::SemiColon {
                    return Err(format!("expected `SemiColon` but found: {:?}", curr));
                }
                Ok(Stmt::Return(e))
            }
            Err(err) => Err(err),
        }
    }

    fn parse_expr(&mut self, min_bp: f32) -> Result<Expr, String> {
        let curr = self.next();

        let mut lhs = match curr {
            Token::Number(num) => {
                let value = num
                    .parse::<i32>()
                    .map_err(|_| format!("failed to parse `{}` to i32", num))?;

                Expr::Int32(value)
            }

            Token::String(str) => Expr::String(str),
            Token::Identifier(ident) => Expr::Ident(ident),

            Token::LeftParen => {
                let expr = self.parse_expr(0.0)?;

                if self.next() != Token::RightParen {
                    return Err("expected `RightParen`".to_string());
                }

                expr
            }

            _ => {
                return Err(format!("unexpected token in expression: {:?}", curr));
            }
        };

        loop {
            let c = self.peek();
            match c {
                // FIX: EOF should not break always (it can return error)
                Token::SemiColon | Token::RightParen | Token::Eof => break,
                Token::Plus | Token::Minus | Token::Star | Token::ForwardSlash | Token::Modulo => {
                    let (lbp, rbp, op) = c
                        .bin_op()
                        .ok_or_else(|| format!("expected `Operator` but found: {:?}", c))?;
                    if lbp < min_bp {
                        break;
                    }
                    self.next();
                    let rhs = self.parse_expr(rbp)?;
                    lhs = Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: lhs,
                        op,
                        right: rhs,
                    }));
                }
                _ => return Err(format!("unexpected token in expression: {:?}", c)),
            }
        }

        return Ok(lhs);
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.idx).unwrap_or(&Token::Eof).clone()
    }

    fn next(&mut self) -> Token {
        let curr = self.peek().clone();
        self.idx += 1;
        curr
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
            err: None,
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
            err: None,
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
            err: None,
        };

        assert_eq!(parser.ast, good_ast);
    }
}
