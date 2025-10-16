use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // punctuation
    LAngle,
    RAngle,
    Slash,
    Equals,
    LBrace,
    RBrace,
    // literals / identifiers
    Ident(String),
    Str(String),
    Text(String),
    // spacing/comments (skipped in output but tracked for spans)
    Whitespace,
    Comment(String),
    // meta
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub col: usize,
    pub offset: usize,
    pub len: usize,
}

impl Span {
    pub fn end_offset(&self) -> usize {
        self.offset + self.len
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::LAngle => write!(f, "<"),
            TokenKind::RAngle => write!(f, ">"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Ident(name) => write!(f, "identifier `{}`", name),
            TokenKind::Str(value) => write!(f, "string \"{}\"", value),
            TokenKind::Text(value) => write!(f, "text `{}`", value),
            TokenKind::Whitespace => write!(f, "whitespace"),
            TokenKind::Comment(_) => write!(f, "comment"),
            TokenKind::Eof => write!(f, "end of file"),
        }
    }
}
