#[derive(Debug, Clone, Copy)]
pub enum IdentType {
    DataType(DataType),
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    CharPtr,
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Return,
}

pub fn get_ident_type(ident: &str) -> Option<IdentType> {
    match ident {
        "int" => Some(IdentType::DataType(DataType::Int)),
        "void" => Some(IdentType::DataType(DataType::Void)),
        "char*" => Some(IdentType::DataType(DataType::CharPtr)),
        "return" => Some(IdentType::Keyword(Keyword::Return)),
        _ => None,
    }
}
