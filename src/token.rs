#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Token {
    PLUS,                 // use for add operation (+)
    MINUS,                // use for subtract operation (-)
    STAR,                 // use for multiplication operation (*)
    MODULO,               // use for divide operation to get remainder (%)
    QUESTION,             // use for ternary operator (?)
    COLON,                // use for ternary operator (:)
    SEMI_COLON,           // use to represent end of a statement (;)
    DOUBLE_STAR,          // use for power operation (**)
    BACK_SLASH,           // use for divide operation (\)
    DOUBLE_FORWARD_SLASH, // use for comment (//)
    EQUAL,                // use for assignment operation (=)
    EQUAL_EQUAL,          // use for comparison operation (==)

    LEFT_PAREN,     // (
    RIGHT_PAREN,    // )
    LEFT_CURLY_BR,  // {
    RIGHT_CURLY_BR, // }
    LEFT_BR,        // [
    RIGHT_BR,       // ]

    STRING(String),     // string
    NUMBER(String),     // int, float, double, etc
    IDENTIFIER(String), // variables, functions

    INVALID(char), // store invalid token
}
