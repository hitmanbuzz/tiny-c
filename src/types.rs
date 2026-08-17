use std::{collections::HashMap, sync::LazyLock};

#[derive(Debug, Clone, Copy)]
pub enum IdentType {
    DATA_TYPE(DataType),
    KEYWORD(Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    INT,
    VOID,
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    RETURN,
}

pub static IDENTIFIERS: LazyLock<HashMap<&'static str, IdentType>> = LazyLock::new(|| {
    HashMap::from([
        ("int", IdentType::DATA_TYPE(DataType::INT)),
        ("void", IdentType::DATA_TYPE(DataType::VOID)),
        ("return", IdentType::KEYWORD(Keyword::RETURN)),
    ])
});
