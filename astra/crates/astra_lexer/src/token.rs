use lasso::Spur;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    If,
    Else,
    While,
    For,
    Let,
    Func,
    Return,
    True,
    False,

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
    EqualEqual,   // for ==
    NotEqual,     // for !=
    Less,         // for <
    Greater,      // for >
    LessEqual,    // for <=
    GreaterEqual, // for >=

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
