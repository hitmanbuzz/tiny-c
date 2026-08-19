use crate::{
    ast::{Ast, Block, Expr, FunctionDef, Node, Stmt},
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
                    curr = self.next();
                    if curr != Token::RightParen {
                        return Err(format!("expected `RightParen` but found: {:?}", curr));
                    }

                    curr = self.next();
                    if curr != Token::LeftCurlyBr {
                        return Err(format!("expected `LeftCurlyBr` but found: {:?}", curr));
                    }

                    let func = self.parse_func_body(data_type, name.as_str());
                    match func {
                        Ok(f) => Ok(Node::FuncDef(f)),
                        Err(err) => Err(err),
                    }
                }
                Token::Equal => {
                    todo!("variable statement")
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

    fn parse_func_body(&mut self, data_type: DataType, name: &str) -> Result<FunctionDef, String> {
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
                todo!("data_type ({:?})", data_type)
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
            _ => Err(format!("expected `Number` but found: {:?}", curr)),
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
