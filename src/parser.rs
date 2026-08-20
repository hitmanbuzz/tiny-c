use crate::{
    ast::{Ast, Block, Expr, ExprIdent, FunctionDef, Node, Stmt, VarStmt},
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
            IdentType::DataType(data_type) => self.parse_data_type(data_type),
            IdentType::Keyword(keyword) => {
                return Err(format!(
                    "expected `DataType` but found: `Keyword ({:?})`",
                    keyword
                ));
            }
        }
    }

    fn parse_data_type(&mut self, data_type: DataType) -> Result<Node, String> {
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
        let expr: Expr;

        match self.peek() {
            Token::Identifier(ident) => {
                let result = Expr::Ident(ExprIdent::Var(ident));
                expr = result;
                self.next(); // move the current token to SemiColon
            }
            _ => {
                let result = self.parse_expr();
                match result {
                    Ok(e) => expr = e,
                    Err(err) => return Err(err),
                }
            }
        }

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

        let block = self.parse_block();
        match block {
            Ok(b) => {
                return Ok(FunctionDef {
                    name: name.to_string(),
                    params: Vec::new(),
                    body: b,
                    return_type: data_type,
                });
            }
            Err(err) => return Err(err),
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut block = Block { stmts: Vec::new() };

        while self.peek() != Token::Eof && self.peek() != Token::RightCurlyBr {
            let stmt = self.parse_stmt();
            match stmt {
                Ok(s) => block.stmts.push(s),
                Err(err) => return Err(err),
            }
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
                let result = self.parse_data_type(*data_type);
                match result {
                    Ok(node) => match node {
                        Node::FuncDef(f) => {
                            return Err(format!("unexpected function within a function: {:?}", f));
                        }
                        Node::Var(var_stmt) => return Ok(Stmt::Var(var_stmt)),
                    },
                    Err(err) => return Err(err),
                }
            }
            IdentType::Keyword(keyword) => match keyword {
                Keyword::Return => {
                    let return_stmt = self.parse_return_stmt();

                    match return_stmt {
                        Ok(stmt) => {
                            let curr = self.next();
                            if curr != Token::RightCurlyBr {
                                return Err(format!(
                                    "expected `RightCurlyBr` but found: {:?}",
                                    curr
                                ));
                            }

                            return Ok(stmt);
                        }
                        Err(err) => return Err(err),
                    }
                }
            },
        }
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr();

        match expr {
            Ok(e) => {
                let curr = self.next();
                if curr != Token::SemiColon {
                    return Err(format!("expected `SemiColon` but found: {:?}", curr));
                }

                Ok(Stmt::Return(e))
            }
            Err(err) => return Err(err),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let curr = self.next();
        match curr {
            Token::Number(num) => {
                let num_value = num.parse::<i32>();
                match num_value {
                    Ok(value) => return Ok(Expr::Int32(value)),
                    Err(_) => return Err(format!("failed to parse `{}` to i32", num)),
                }
            }
            Token::String(str) => return Ok(Expr::String(str)),
            _ => Err(format!("expected expression but found: `{:?}`", curr)),
        }
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
    use crate::lexer::Lexer;

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
}
