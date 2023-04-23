#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Let
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Colon,
    Keyword(Keyword),
    Identifier(String),
    Integer(i32)
}