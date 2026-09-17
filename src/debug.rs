use crate::lexer::Lexer;
use crate::parser::Parser;

impl<'l> Lexer<'l> {
    pub fn print(&self) {
        println!("―――――――――――――――――――――――――――――― LEXER ――――――――――――――――――――――――――――――");
        println!("{:#?}\n", self.tokens);
        println!("――――――――――――――――――――――――――――――  END  ――――――――――――――――――――――――――――――\n\n");
    }
}

impl Parser {
    pub fn print(&self) {
        println!("――――――――――――――――――――――――――――――  AST  ――――――――――――――――――――――――――――――");
        for node in self.ast.nodes.iter() {
            println!("{}", node);
        }
        println!("――――――――――――――――――――――――――――――  END  ――――――――――――――――――――――――――――――\n");
    }
}
