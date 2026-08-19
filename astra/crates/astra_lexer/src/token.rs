use lasso::Spur;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Func,
    Return,

    // values/names
    Identifier(Spur),
    Integer(i64),
    StringLiteral(String),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,

    // Special
    EoF,
}
