use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone, Copy)]
pub enum IdentType {
    DataType(DataType),
    Keyword(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Int,
    Void,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Return,
}

pub static IDENTIFIERS: LazyLock<HashMap<&'static str, IdentType>> = LazyLock::new(|| {
    HashMap::from([
        ("int", IdentType::DataType(DataType::Int)),
        ("void", IdentType::DataType(DataType::Void)),
        ("return", IdentType::Keyword(Keyword::Return)),
    ])
});
