#[derive(Debug, Clone, Copy)]
pub enum IdentType {
    DataType(DataType),
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// 32 bit signed integer (i32)
    Int32,
    Bool,
    String,
    Void,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Return,
    If,
    Else,
}
