use std::fmt::Display;

use crate::lexer::{LexError, Lexer};
use crate::parser::{ParseError, Parser};
use crate::token::Token;

impl std::error::Error for LexError {}
impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[LEXER ERROR] [Line: {} | Pos: {}]: {}",
            self.line, self.pos, self.msg
        )
    }
}

impl std::error::Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[PARSER ERROR] [Line: {} | Pos: {}]: {}",
            self.line, self.pos, self.msg
        )
    }
}

#[allow(dead_code)]
impl<'l> Lexer<'l> {
    pub fn print(&self) {
        println!("―――――――――――――――――――――――――――――― LEXER ――――――――――――――――――――――――――――――");
        println!("{:#?}\n", self.tokens);
        println!("――――――――――――――――――――――――――――――  END  ――――――――――――――――――――――――――――――\n\n");
    }

    /// print out all invalid token
    pub fn print_invalid(&self) {
        self.tokens
            .iter()
            .filter(|t| matches!(t.token, Token::Invalid(_)))
            .for_each(|t| println!("{:?}", t.token));
    }
}

#[allow(dead_code)]
impl Parser {
    pub fn print(&self) {
        println!("――――――――――――――――――――――――――――――  AST  ――――――――――――――――――――――――――――――");
        println!("{}", self.ast);
        println!("――――――――――――――――――――――――――――――  END  ――――――――――――――――――――――――――――――\n");
    }
}
