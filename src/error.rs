use crate::lexer::Lexer;

pub trait ErrorMsg {
    fn print_err(&self);
}

pub enum LexerError {
    // identifier starting with number
    //
    // eg: 1name, 67work
    IDENT_START_WITH_NUM(String),
    // float number with more than 1 dot
    //
    // eg: 5..6, 5.6.6
    NUM_HAS_MORE_DOTS(String),
    // string doesn't have a end quotation
    //
    // eg: "Hello World
    BAD_STRING(String),
}

impl LexerError {
    fn print(&self) {
        match self {
            LexerError::IDENT_START_WITH_NUM(token) => {
                eprintln!("[ERROR] `IDENTIFIER` start with a number: `{}`", token)
            }
            LexerError::NUM_HAS_MORE_DOTS(token) => {
                eprintln!("[ERROR] `NUMBER` contains more than 1 dot: `{}`", token)
            }
            LexerError::BAD_STRING(token) => {
                eprintln!(
                    "[ERROR] `IDENTIFIER` cannot start with a number: `{}`",
                    token
                )
            }
        }
    }
}

impl<'l> ErrorMsg for Lexer<'l> {
    fn print_err(&self) {
        if self.errors.is_empty() {
            return;
        }

        for err in self.errors.iter() {
            err.print();
        }
    }
}
