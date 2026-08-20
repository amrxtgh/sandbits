use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar { char: char, position: usize },
    InvalidInteger { string: String, position: usize },
    UnterminatedString { position: usize },
    InvalidEscape { position: usize },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar { char, position } => {
                write!(
                    f,
                    "unexpected character '{}' at position {}",
                    char, position,
                )
            }
            LexerError::InvalidInteger { string, position } => {
                write!(f, "invalid integer '{}' at position {}", string, position)
            }
            LexerError::UnterminatedString { position } => {
                write!(f, "unterminated string starting at position {}", position)
            }
            LexerError::InvalidEscape { position } => {
                write!(f, "Invalid escape at position {}", position)
            }
        }
    }
}
impl std::error::Error for LexerError {}
