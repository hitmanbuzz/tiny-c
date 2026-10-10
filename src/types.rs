#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Bool,
    String,
    Void,

    True,
    False,

    Return,
    If,
    Else,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// 32 bit signed integer (i32)
    Int32,
    Bool,
    String,
    Void,
}

impl Keyword {
    /// keyword -> datatype
    pub fn to_data_type(&self) -> Option<DataType> {
        match self {
            Keyword::Int => Some(DataType::Int32),
            Keyword::Bool => Some(DataType::Bool),
            Keyword::String => Some(DataType::String),
            Keyword::Void => Some(DataType::Void),
            _ => None,
        }
    }
}
