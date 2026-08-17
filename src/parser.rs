use crate::{
    ast::{Ast, Block, Expr, FunctionDef, Stmt},
    token::Token,
    types::{DataType, IDENTIFIERS, IdentType, Keyword},
};

pub struct Parser {
    pub ast: Ast,
    pub err_msg: String,
    tokens: Vec<Token>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            idx: 0,
            err_msg: String::new(),
            ast: Ast { f: None },
        }
    }

    pub fn parse(&mut self) {
        match self.parse_func_def() {
            Ok(f) => self.ast = Ast { f: Some(f) },
            Err(err) => self.err_msg = err,
        }
    }

    fn parse_func_def(&mut self) -> Result<FunctionDef, String> {
        if !matches!(self.peek(), Token::IDENTIFIER(_)) {
            return Err(format!(
                "expected `IDENTIFIER` for `func_return_type` but found: `{:?}`",
                self.peek()
            ));
        }

        // we are sure this is an `IDENTIFIER`
        let func_return_type = self.next().ident_name();
        let ident_type = self.get_ident(func_return_type.as_str()).ok_or_else(|| {
            format!(
                "conversion of `IDENTIFIER ({})` to its distinct type is not implemented",
                func_return_type
            )
        })?;

        match *ident_type {
            IdentType::DATA_TYPE(data_type) => {
                if data_type != DataType::INT {
                    return Err(format!(
                        "expected `INT` data type but found: `{:?}`",
                        data_type
                    ));
                }

                if let Token::IDENTIFIER(func_name) = self.next() {
                    match self.next() {
                        Token::LEFT_PAREN => {}
                        _ => {
                            return Err(format!(
                                "expected `LEFT_PAREN` but found: {:?}",
                                self.peek_prev(),
                            ));
                        }
                    }

                    match self.next() {
                        Token::RIGHT_PAREN => {}
                        _ => {
                            return Err(format!(
                                "expected `RIGHT_PAREN` but found: {:?}",
                                self.peek_prev(),
                            ));
                        }
                    }

                    match self.next() {
                        Token::LEFT_CURLY_BR => {}
                        _ => {
                            return Err(format!(
                                "expected `LEFT_CURLY_BR` but found: {:?}",
                                self.peek_prev(),
                            ));
                        }
                    }

                    let body = self.parse_block()?;

                    return Ok(FunctionDef {
                        name: func_name,
                        params: vec![],
                        body: body,
                        return_type: data_type,
                    });
                } else {
                    return Err(format!(
                        "expected `IDENTIFIER` for `func_name` but found: `{:?}`",
                        self.peek_prev()
                    ));
                }
            }
            IdentType::KEYWORD(keyword) => {
                println!("3");
                Err(format!("expected `IDENTIFIER` but found: `{:?}`", keyword))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        let mut block = Block { stmts: Vec::new() };

        while self.peek() != Token::EOF && self.peek() != Token::RIGHT_CURLY_BR {
            let stmt = self.parse_stmt()?;
            block.stmts.push(stmt);
        }

        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        if !matches!(self.peek(), Token::IDENTIFIER(_)) {
            return Err(format!(
                "expected `IDENTIFIER` on parse_stmt but found {:?}",
                self.peek_prev()
            ));
        }

        // we are sure this is ident
        let ident = self.next().ident_name();
        let ident_type = self.get_ident(ident.as_str()).ok_or_else(|| {
            format!(
                "conversion of IDENTIFIER({}) to its distinct type is not implemented",
                ident
            )
        })?;

        match ident_type {
            IdentType::DATA_TYPE(data_type) => {
                todo!("data_type ({:?})", data_type)
            }
            IdentType::KEYWORD(keyword) => match keyword {
                Keyword::RETURN => {
                    let return_stmt = self.parse_return_stmt();

                    if self.next() != Token::RIGHT_CURLY_BR {
                        return Err(format!(
                            "expected `RIGHT_CURLY_BR` but found: {:?}",
                            self.peek()
                        ));
                    }

                    return return_stmt;
                }
            },
        }
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;

        if self.next() != Token::SEMI_COLON {
            return Err(format!(
                "expected `SEMI_COLON` but found: {:?}",
                self.peek()
            ));
        }

        Ok(Stmt::RETURN(expr))
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let curr = self.next();
        match curr {
            Token::NUMBER(num) => {
                let num_value = num.parse::<i32>();
                match num_value {
                    Ok(value) => return Ok(Expr::INT32(value)),
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => Err(format!("expected `NUMBER` but found: {:?}", curr)),
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.idx).unwrap_or(&Token::EOF).clone()
    }

    /// get the previous Token (`self.idx - 1`)
    fn peek_prev(&self) -> &Token {
        self.tokens.get(self.idx - 1).unwrap_or(&Token::EOF)
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
