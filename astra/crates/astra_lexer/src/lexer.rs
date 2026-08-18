use crate::token::Token;

pub struct Lexer {
    // contains the character we are scanning
    //  l e t x = 69;
    source: Vec<char>,
    // tells us where we are
    // let x = 69;
    // ^
    // then position = 0
    position: usize,
}
impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
        }
    }
    pub fn current(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }
    pub fn advance(&mut self) {
        self.position += 1;
    }
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    // lets make lex produce tokens
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current().is_some() {
            self.skip_whitespace();

            let Some(c) = self.current() else {
                break;
            };
            match c {
                '=' => {
                    tokens.push(Token::Equal);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }
        tokens
    }
}
