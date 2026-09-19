use crate::ast::BinaryOp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Plus,               // use for add operation (+)
    Minus,              // use for subtract operation (-)
    Star,               // use for multiplication operation (*)
    Modulo,             // use for divide operation to get remainder (%)
    Question,           // use for ternary operator (?)
    Colon,              // use for ternary operator (:)
    SemiColon,          // use to represent end of a statement (;)
    StarStar,           // use for power operation (**)
    ForwardSlash,       // use for divide operation (/)
    DoubleForwardSlash, // use for comment (//)
    Equal,              // use for assignment operation (=)
    EqualEqual,         // use for comparison operation (==)

    LeftParen,    // (
    RightParen,   // )
    LeftCurlyBr,  // {
    RightCurlyBr, // }
    LeftBr,       // [
    RightBr,      // ]

    String(String),     // string
    Number(String),     // int, float, double, etc
    Identifier(String), // variables, functions

    Invalid(char), // store invalid token

    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenData {
    pub token: Token,
    pub line: usize,
    pub pos: usize,
}

impl Default for TokenData {
    fn default() -> Self {
        Self {
            token: Token::Eof,
            line: Default::default(),
            pos: Default::default(),
        }
    }
}

impl Token {
    // (left_binding_power, right_binding_power, BinaryOp)
    pub fn bin_op(&self) -> Option<(f32, f32, BinaryOp)> {
        // TODO: add other operator
        match self {
            Token::Plus => Some((1.0, 1.1, BinaryOp::Add)),
            Token::Minus => Some((1.0, 1.1, BinaryOp::Sub)),

            Token::Star => Some((2.0, 2.1, BinaryOp::Mul)),
            Token::ForwardSlash => Some((2.0, 2.1, BinaryOp::Div)),
            Token::Modulo => Some((2.0, 2.1, BinaryOp::Modulo)),
            _ => None,
        }
    }
}
