#[derive(Debug, PartialEq)]
pub enum Token {
    Let,
    Identifier(String),
    Integer(i64),

    Equal,
    Semicolon,
}
