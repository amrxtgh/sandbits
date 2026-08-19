use crate::buffer::Buffer;
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
}
impl Lexer {
    // all the characters
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            position: 0,
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
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let Some(c) = self.current() else {
            return Token::EoF;
        };
        match c {
            '=' => {
                self.advance();
                Token::Equal
            }
            ';' => {
                self.advance();
                Token::Semicolon
            }
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '*' => {
                self.advance();
                Token::Star
            }
            '/' => {
                self.advance();
                Token::Slash
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let text = self.read_identifier();
                self.keyword_or_identifier(text)
            }
            _ if c.is_ascii_digit() => {
                let number = self.read_number();
                Token::Integer(number)
            }
            _ => {
                panic!("unexpected character: {c}");
            }
        }
    }
    pub fn tokenize(&mut self) -> Buffer<Token> {
        let mut tokens = Buffer::new();
        loop {
            let token = self.next_token();
            let done = token == Token::EoF;

            tokens.push(token);
            if done {
                break;
            }
        }
        tokens
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
    fn keyword_or_identifier(&self, text: String) -> Token {
        match text.as_str() {
            "let" => Token::Let,
            "func" => Token::Func,
            "return" => Token::Return,
            _ => Token::Identifier(text),
        }
    }
    fn read_number(&mut self) -> i64 {
        let start = self.position;

        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        let text: String = self.source[start..self.position].iter().collect();
        text.parse().expect("lexer produced invalud integer")
    }
}
