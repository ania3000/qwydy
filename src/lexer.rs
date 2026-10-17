use crate::keywords::keyword_kind;
use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(ch)
    }

    fn make_token(&self, kind: TokenKind, line: usize, column: usize) -> Token {
        Token { kind, line, column }
    }

    fn is_identifier_start(ch: char) -> bool {
        ch == '_' || ch.is_alphabetic() || ('\u{0400}'..='\u{04FF}').contains(&ch)
    }

    fn is_identifier_continue(ch: char) -> bool {
        Self::is_identifier_start(ch) || ch.is_ascii_digit()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            // Комментарии начинаются с //
            if ch == '/' && self.peek_next() == Some('/') {
                while let Some(c) = self.peek() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                continue;
            }

            let line = self.line;
            let column = self.column;

            let kind = if ch.is_ascii_digit() {
                let mut number = String::new();

                while let Some(c) = self.peek() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    number.push(self.advance().unwrap());
                }

                TokenKind::Number(number)
            } else if Self::is_identifier_start(ch) {
                let mut name = String::new();

                while let Some(c) = self.peek() {
                    if !Self::is_identifier_continue(c) {
                        break;
                    }
                    name.push(self.advance().unwrap());
                }

                keyword_kind(&name).unwrap_or(TokenKind::Identifier(name))
            } else {
                self.advance();

                match ch {
                    '{' => TokenKind::LeftBrace,
                    '}' => TokenKind::RightBrace,
                    '+' => TokenKind::Plus,
                    '-' => TokenKind::Minus,
                    '*' => TokenKind::Star,
                    '/' => TokenKind::Slash,
                    '(' => TokenKind::LeftParen,
                    ')' => TokenKind::RightParen,
                    ';' => TokenKind::Semicolon,
                    '=' if self.peek() == Some('=') => {
                        self.advance();
                        TokenKind::EqualEqual
                    }
                    '=' => TokenKind::Equal,
                    _ => continue,
                }
            };

            tokens.push(self.make_token(kind, line, column));
        }

        tokens.push(self.make_token(TokenKind::Eof, self.line, self.column));

        tokens
    }
}
