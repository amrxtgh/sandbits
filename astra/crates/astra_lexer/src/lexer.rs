use crate::buffer::Buffer;
use crate::error::LexerError;
use crate::interner::Interner;
use crate::token::Token;

pub struct Lexer {
    // contains the character we are scanning
    //  l e t x = 6 9;
    source: Vec<char>,
    // tells us where we are
    // let x = 69;
    // ^
    // then position = 0
    position: usize,

    // here our lexer owns the interner so the interned strings belongs to the lifetime of that
    // lexer
    interner: Interner,
}
impl Lexer {
    // all the characters
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
            interner: Interner::new(),
        }
    }
    // what characters lexer is looking at
    fn current(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    // move to the next character
    fn advance(&mut self) {
        self.position += 1;
    }

    // inspect the another character
    fn peek(&self) -> Option<char> {
        self.source.get(self.position + 1).copied()
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    // lets make lex produce tokens
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let Some(c) = self.current() else {
            return Ok(Token::EoF);
        };
        match c {
            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    Ok(Token::EqualEqual)
                } else {
                    self.advance();
                    Ok(Token::Equal)
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    Ok(Token::NotEqual)
                } else {
                    Err(LexerError::UnexpectedChar {
                        char: '!',
                        position: self.position,
                    })
                }
            }
            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    Ok(Token::LessEqual)
                } else {
                    self.advance();
                    Ok(Token::Less)
                }
            }
            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.advance();
                    Ok(Token::GreaterEqual)
                } else {
                    self.advance();
                    Ok(Token::Greater)
                }
            }

            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '-' => {
                self.advance();
                Ok(Token::Minus)
            }
            '*' => {
                self.advance();
                Ok(Token::Star)
            }
            '/' => {
                if self.peek() == Some('/') {
                    self.advance();
                    self.advance();
                    while let Some(c) = self.current() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    // i use recursion for the first time lessgo dsa
                    Ok(self.next_token()?)
                } else {
                    self.advance();
                    Ok(Token::Slash)
                }
            }
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            '{' => {
                self.advance();
                Ok(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(Token::RightBrace)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '"' => {
                let string = self.read_string()?;
                Ok(Token::StringLiteral(string))
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let text = self.read_identifier();
                Ok(self.keyword_or_identifier(text))
            }
            _ if c.is_ascii_digit() => {
                let number = self.read_number();
                Ok(Token::Integer(number?))
            }
            _ => Err(LexerError::UnexpectedChar {
                char: c,
                position: self.position,
            }),
        }
    }
    pub fn tokenize(&mut self) -> Result<Buffer<Token>, LexerError> {
        let mut tokens = Buffer::new();
        loop {
            let token = self.next_token()?;
            let done = token == Token::EoF;

            tokens.push(token);

            if done {
                break;
            }
        }
        Ok(tokens)
    }
    fn read_string(&mut self) -> Result<String, LexerError> {
        self.advance();
        let start = self.position;
        let mut result = String::new();

        while let Some(c) = self.current() {
            // get some character at the current lexer position
            match c {
                '"' => {
                    // found the closing quote of the string
                    self.advance();
                    // consume the closing quote and move to the next character
                    return Ok(result);
                    // the string is complete
                }
                '\\' => match self.peek() {
                    // found the back slash
                    Some('n') => {
                        // source contain "\n"
                        result.push('\n');
                        // consume the \ character
                        self.advance();
                        // consume the n character
                        self.advance();
                    }
                    Some('"') => {
                        result.push('"');
                        self.advance();
                        self.advance();
                    }
                    Some('\\') => {
                        result.push('\\');
                        self.advance();
                        self.advance();
                    }
                    _ => {
                        return Err(LexerError::InvalidEscape {
                            position: self.position,
                        });
                    }
                },
                _ => {
                    // for normal character
                    result.push(c);
                    self.advance();
                }
            }
        }
        Err(LexerError::UnterminatedString { position: start })
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.current() {
            if c.is_ascii_alphabetic() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.source[start..self.position].iter().collect()
    }
    fn keyword_or_identifier(&mut self, text: String) -> Token {
        match text.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "let" => Token::Let,
            "func" => Token::Func,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => {
                let symbol = self.interner.intern(&text);
                Token::Identifier(symbol)
            }
        }
    }
    fn read_number(&mut self) -> Result<i64, LexerError> {
        let start = self.position;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        let text: String = self.source[start..self.position].iter().collect();
        match text.parse::<i64>() {
            Ok(number) => Ok(number),
            Err(_) => Err(LexerError::InvalidInteger {
                string: text,
                position: start,
            }),
        }
    }
}
