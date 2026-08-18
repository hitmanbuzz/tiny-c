#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,               // use for add operation (+)
    Minus,              // use for subtract operation (-)
    Star,               // use for multiplication operation (*)
    Modulo,             // use for divide operation to get remainder (%)
    Question,           // use for ternary operator (?)
    Colon,              // use for ternary operator (:)
    SemiColon,          // use to represent end of a statement (;)
    StarStar,           // use for power operation (**)
    BackSlash,          // use for divide operation (\)
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

impl Token {
    pub fn ident_name(self) -> String {
        match self {
            Token::Identifier(name) => name,
            _ => panic!("expected identifier"),
        }
    }
}
