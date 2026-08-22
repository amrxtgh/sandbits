use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedChar {
        character: char,
        line: usize,
        column: usize,
    },
    InvalidInteger {
        string: String,
        line: usize,
        column: usize,
    },
    UnterminatedString {
        line: usize,
        column: usize,
    },
    InvalidEscape {
        line: usize,
        column: usize,
    },
}

// i need to check this
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar {
                character,
                line,
                column,
            } => {
                write!(
                    f,
                    "unexpected character '{}' at line '{}' column '{}'",
                    character, line, column
                )
            }
            LexerError::InvalidInteger {
                string,
                line,
                column,
            } => {
                write!(
                    f,
                    "invalid integer '{}' at line '{}' column '{}'",
                    string, line, column
                )
            }
            LexerError::UnterminatedString { line, column } => {
                write!(
                    f,
                    "unterminated string starting at line '{}' column '{}'",
                    line, column
                )
            }
            LexerError::InvalidEscape { line, column } => {
                write!(f, "Invalid escape at line '{}' column '{}'", line, column)
            }
        }
    }
}
impl std::error::Error for LexerError {}
