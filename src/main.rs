use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let source = "
        int main() {
            return 12.45;
        }
    ";

    let mut lexer = Lexer::new(source);
    lexer.tokenize();

    println!("{:#?}", lexer.tokens);
}
