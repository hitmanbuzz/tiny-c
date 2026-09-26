use crate::ast::BinaryOp;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Plus,         // +
    Minus,        // -
    Star,         // *
    ForwardSlash, // /
    Modulo,       // %

    And, // &&
    Or,  // ||

    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    BitNot, // ~
    BitLS,  // <<
    BitRS,  // >>

    Not, // !

    Question,           // ?
    Colon,              // :
    SemiColon,          // ;
    StarStar,           // **
    DoubleForwardSlash, // //

    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    Equal,        // =
    NotEqual,     // !=
    EqualEqual,   // ==

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
            Token::Or => Some((1.0, 1.1, BinaryOp::Or)),
            Token::And => Some((2.0, 2.1, BinaryOp::And)),

            Token::BitOr => Some((11.0, 11.1, BinaryOp::BitOr)),
            Token::BitXor => Some((12.0, 12.1, BinaryOp::BitXor)),
            Token::BitAnd => Some((13.0, 13.1, BinaryOp::BitAnd)),
            Token::BitLS => Some((14.0, 14.1, BinaryOp::BitLS)),
            Token::BitRS => Some((15.0, 15.1, BinaryOp::BitRS)),

            Token::NotEqual => Some((21.0, 21.1, BinaryOp::NotEqual)),
            Token::EqualEqual => Some((21.0, 21.1, BinaryOp::EqualEqual)),

            Token::Less => Some((22.0, 22.1, BinaryOp::Less)),
            Token::Greater => Some((22.0, 22.1, BinaryOp::Greater)),
            Token::LessEqual => Some((22.0, 22.1, BinaryOp::LessEqual)),
            Token::GreaterEqual => Some((22.0, 22.1, BinaryOp::GreaterEqual)),

            Token::Plus => Some((41.0, 41.1, BinaryOp::Add)),
            Token::Minus => Some((41.0, 41.1, BinaryOp::Sub)),

            Token::Star => Some((51.0, 51.1, BinaryOp::Mul)),
            Token::ForwardSlash => Some((51.0, 51.1, BinaryOp::Div)),
            Token::Modulo => Some((51.0, 51.1, BinaryOp::Modulo)),
            _ => None,
        }
    }
}
