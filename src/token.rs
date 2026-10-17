#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Number(String),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    LeftParen,
    RightParen,
    Semicolon,
    Eof,
    Function,
    Let,
    If,
    Else,
    LeftBrace,
    RightBrace,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
