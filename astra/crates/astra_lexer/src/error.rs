use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar { char: char, position: usize },
    InvalidInteger { string: String, position: usize },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar { char, position } => write!(
                f,
                "unexpected character '{}' at position {}",
                char, position,
            ),
            LexerError::InvalidInteger { string, position } => {
                write!(f, "invalid integer '{}' at position {}", string, position)
            }
        }
    }
}
impl std::error::Error for LexerError {}
