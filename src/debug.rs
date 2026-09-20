use crate::lexer::Lexer;
use crate::parser::Parser;

#[allow(dead_code)]
impl<'l> Lexer<'l> {
    pub fn print(&self) {
        println!("―――――――――――――――――――――――――――――― LEXER ――――――――――――――――――――――――――――――");
        println!("{:#?}\n", self.tokens);
        println!("――――――――――――――――――――――――――――――  END  ――――――――――――――――――――――――――――――\n\n");
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
