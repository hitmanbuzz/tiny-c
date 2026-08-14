use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let source = r#"
        int main() {
            char* a = "Hello World";
            return 12.45;
        }
    "#;

    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    println!("{:#?}", lexer.tokens);
}
