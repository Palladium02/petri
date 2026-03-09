use std::{iter::Peekable, range::Range, str::Chars};

pub type SpannedToken = (Token, Range<usize>);

#[derive(Clone)]
pub enum Token {
    // Keywords
    Place,
    Transition,
    Tokens,

    Identifier(String),

    // Literals
    Integer(usize),
    String(String),

    // Delimiters
    Arrow,
    LeftBracket,
    RightBracket,
    Semicolon,
    Equals,

    // Special
    Unexpected(char),
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        match self {
            Token::Place => TokenKind::Place,
            Token::Transition => TokenKind::Transition,
            Token::Tokens => TokenKind::Tokens,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::Integer(_) => TokenKind::Integer,
            Token::String(_) => TokenKind::String,
            Token::Arrow => TokenKind::Arrow,
            Token::LeftBracket => TokenKind::LeftBracket,
            Token::RightBracket => TokenKind::RightBracket,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Equals => TokenKind::Equals,
            Token::Unexpected(_) => TokenKind::Unexpected,
        }
    }
}

#[derive(PartialEq)]
pub enum TokenKind {
    // Keywords
    Place,
    Transition,
    Tokens,

    Identifier,

    // Literals
    Integer,
    String,

    // Delimiters
    Arrow,
    LeftBracket,
    RightBracket,
    Semicolon,
    Equals,

    // Special
    Unexpected,
}

#[derive(Clone)]
pub struct Lexer<'c> {
    input: Peekable<Chars<'c>>,
    position: usize,
}

impl<'c> Lexer<'c> {
    pub fn new(input: &'c str) -> Self {
        Self {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<SpannedToken> {
        let current_position = self.position;
        let spanned_token = match self.next_char()? {
            '[' => self.emit_token(current_position, Token::LeftBracket),
            ']' => self.emit_token(current_position, Token::RightBracket),
            ';' => self.emit_token(current_position, Token::Semicolon),
            '=' => self.emit_token(current_position, Token::Equals),
            '-' => {
                if self.next_char_if(|c| c == '>').is_some() {
                    self.emit_token(current_position, Token::Arrow)
                } else {
                    self.emit_token(current_position, Token::Unexpected('-'))
                }
            }
            '#' => {
                while self.next_char_if(|c| c != '\n').is_some() {}
                self.next_token()?
            }
            '"' => {
                let mut string = String::new();

                while let Some(c) = self.next_char() {
                    match c {
                        '"' => break,
                        '\\' => {
                            if let Some(c) = self.next_char() {
                                match c {
                                    'n' => string.push('\n'),
                                    't' => string.push('\t'),
                                    'r' => string.push('\r'),
                                    '\\' => string.push('\\'),
                                    '"' => string.push('"'),
                                    _ => string.push(c),
                                }
                            }
                        }
                        c => string.push(c),
                    }
                }

                self.emit_token(current_position, Token::String(string))
            }
            c if c.is_whitespace() => {
                while self.next_char_if(char::is_whitespace).is_some() {}
                self.next_token()?
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::from(c);
                while let Some(c) = self.next_char_if(|c| c.is_alphanumeric() || c == '_') {
                    identifier.push(c);
                }

                match identifier.as_str() {
                    "place" => self.emit_token(current_position, Token::Place),
                    "transition" => self.emit_token(current_position, Token::Transition),
                    "tokens" => self.emit_token(current_position, Token::Tokens),
                    _ => self.emit_token(current_position, Token::Identifier(identifier)),
                }
            }
            c if c.is_numeric() => {
                let mut number = String::from(c);
                while let Some(c) = self.next_char_if(char::is_numeric) {
                    number.push(c);
                }

                self.emit_token(
                    current_position,
                    Token::Integer(
                        number
                            .parse()
                            .expect("This cannot fail due to how we constructed `number`"),
                    ),
                )
            }
            c => self.emit_token(current_position, Token::Unexpected(c)),
        };

        Some(spanned_token)
    }

    fn next_char(&mut self) -> Option<char> {
        self.next_char_if(|_| true)
    }

    fn next_char_if(&mut self, f: impl FnOnce(char) -> bool) -> Option<char> {
        self.input.next_if(|&c| f(c)).inspect(|c| {
            self.position += c.len_utf8();
        })
    }

    fn emit_token(&self, start: usize, token: Token) -> SpannedToken {
        (token, Range::from(start..self.position))
    }
}

impl Iterator for Lexer<'_> {
    type Item = SpannedToken;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
