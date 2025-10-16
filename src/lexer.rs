use crate::token::{Span, Token, TokenKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexError {
    #[error("invalid token at {0:?}")]
    Invalid(Span),
}

struct Lexer<'a> {
    input: &'a str,
    index: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            match ch {
                '<' => tokens.push(self.consume_single(TokenKind::LAngle)),
                '>' => tokens.push(self.consume_single(TokenKind::RAngle)),
                '/' => {
                    if self.peek_next_char() == Some('/') {
                        self.consume_comment();
                        continue;
                    }
                    tokens.push(self.consume_single(TokenKind::Slash));
                }
                '=' => tokens.push(self.consume_single(TokenKind::Equals)),
                '{' => {
                    let lbrace_token = self.consume_single(TokenKind::LBrace);
                    tokens.push(lbrace_token);
                    self.consume_expression(&mut tokens)?;
                }
                '}' => {
                    // stray closing brace
                    tokens.push(self.consume_single(TokenKind::RBrace));
                }
                '"' => {
                    let (token, _) = self.consume_string()?;
                    tokens.push(token);
                }
                c if c.is_whitespace() => {
                    self.consume_whitespace();
                }
                c if is_ident_start(c) => {
                    let token = self.consume_ident();
                    tokens.push(token);
                }
                _ => {
                    let (token, produced) = self.consume_text();
                    if produced {
                        tokens.push(token);
                    }
                }
            }
        }

        let eof_span = Span {
            line: self.line,
            col: self.col,
            offset: self.index,
            len: 0,
        };
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: eof_span,
        });

        Ok(tokens)
    }

    fn consume_single(&mut self, kind: TokenKind) -> Token {
        let mark = self.mark();
        self.advance_char();
        let span = self.span_from(mark);
        Token { kind, span }
    }

    fn consume_ident(&mut self) -> Token {
        let mark = self.mark();
        self.advance_char();
        while let Some(ch) = self.peek_char() {
            if is_ident_continue(ch) {
                self.advance_char();
            } else {
                break;
            }
        }
        let span = self.span_from(mark);
        let text = &self.input[span.offset..span.end_offset()];
        Token {
            kind: TokenKind::Ident(text.to_string()),
            span,
        }
    }

    fn consume_string(&mut self) -> Result<(Token, String), LexError> {
        let mark = self.mark();
        // skip opening quote
        self.advance_char();
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            match ch {
                '\\' => {
                    self.advance_char();
                    match self.peek_char() {
                        Some('"') => {
                            self.advance_char();
                            value.push('"');
                        }
                        Some('\\') => {
                            self.advance_char();
                            value.push('\\');
                        }
                        Some('n') => {
                            self.advance_char();
                            value.push('\n');
                        }
                        Some('t') => {
                            self.advance_char();
                            value.push('\t');
                        }
                        Some(other) => {
                            self.advance_char();
                            value.push(other);
                        }
                        None => {
                            return Err(LexError::Invalid(self.span_from(mark)));
                        }
                    }
                }
                '"' => {
                    self.advance_char();
                    let span = self.span_from(mark);
                    let token = Token {
                        kind: TokenKind::Str(value.clone()),
                        span,
                    };
                    return Ok((token, value));
                }
                _ => {
                    self.advance_char();
                    value.push(ch);
                }
            }
        }

        Err(LexError::Invalid(self.span_from(mark)))
    }

    fn consume_text(&mut self) -> (Token, bool) {
        let mark = self.mark();
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            if ch == '<' || ch == '{' {
                break;
            }
            value.push(ch);
            self.advance_char();
        }

        let span = self.span_from(mark);
        let trimmed = trim_text(&value);
        if trimmed.is_empty() {
            (
                Token {
                    kind: TokenKind::Whitespace,
                    span,
                },
                false,
            )
        } else {
            (
                Token {
                    kind: TokenKind::Text(trimmed),
                    span,
                },
                true,
            )
        }
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.advance_char();
            } else {
                break;
            }
        }
    }

    fn consume_comment(&mut self) {
        // consume initial '//'
        self.advance_char();
        self.advance_char();
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break;
            }
            self.advance_char();
        }
    }

    fn consume_expression(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexError> {
        let start_span = tokens.last().map(|t| t.span.clone()).unwrap_or(Span {
            line: self.line,
            col: self.col,
            offset: self.index,
            len: 0,
        });

        while let Some(ch) = self.peek_char() {
            if ch == '}' {
                let mark = self.mark();
                self.advance_char();
                let span = self.span_from(mark);
                tokens.push(Token {
                    kind: TokenKind::RBrace,
                    span,
                });
                return Ok(());
            }
            self.advance_char();
        }

        Err(LexError::Invalid(start_span))
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.index..].chars().next()
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut iter = self.input[self.index..].chars();
        iter.next()?;
        iter.next()
    }

    fn advance_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        let len = ch.len_utf8();
        self.index += len;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else if ch == '\r' {
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn mark(&self) -> Mark {
        Mark {
            index: self.index,
            line: self.line,
            col: self.col,
        }
    }

    fn span_from(&self, mark: Mark) -> Span {
        Span {
            line: mark.line,
            col: mark.col,
            offset: mark.index,
            len: self.index - mark.index,
        }
    }
}

struct Mark {
    index: usize,
    line: usize,
    col: usize,
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

fn trim_text(value: &str) -> String {
    let mut text = value.to_string();
    if text.starts_with('\n') {
        text.remove(0);
    }
    if text.ends_with('\n') {
        text.pop();
    }
    text
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    Lexer::new(input).lex()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_simple_element() {
        let tokens = lex("<div>Hello</div>").unwrap();
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::LAngle,
                TokenKind::Ident("div".into()),
                TokenKind::RAngle,
                TokenKind::Text("Hello".into()),
                TokenKind::LAngle,
                TokenKind::Slash,
                TokenKind::Ident("div".into()),
                TokenKind::RAngle,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn lexes_attribute_string() {
        let tokens = lex("<button class=\"cta\"></button>").unwrap();
        let kinds: Vec<_> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(kinds[0], TokenKind::LAngle);
        assert_eq!(kinds[1], TokenKind::Ident("button".into()));
        assert_eq!(kinds[2], TokenKind::Ident("class".into()));
        assert_eq!(kinds[3], TokenKind::Equals);
        assert_eq!(kinds[4], TokenKind::Str("cta".into()));
    }
}
