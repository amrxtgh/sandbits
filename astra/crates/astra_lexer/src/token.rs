#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Func,
    Return,

    // values/names
    Identifier(String),
    Integer(i64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrac,
    RightBrac,
    Semicolon,
    Comma,

    // Special
    EoF,
}
