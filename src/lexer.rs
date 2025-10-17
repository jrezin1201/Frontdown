//! Lexer

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
    in_tag: bool, // <-- track whether we're inside a <...> tag
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            index: 0,
            line: 1,
            col: 1,
            in_tag: false,
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek_char() {
            match ch {
                '<' => tokens.push(self.consume_single(TokenKind::LAngle)),
                '>' => tokens.push(self.consume_single(TokenKind::RAngle)),
                '/' => {
                    if self.peek_next_char() == Some('/') && !self.in_tag {
                        // Only treat // as a line comment when not inside a tag
                        self.consume_comment();
                        continue;
                    }
                    tokens.push(self.consume_single(TokenKind::Slash));
                }
                '=' => tokens.push(self.consume_single(TokenKind::Equals)),
                '{' => {
                    let lbrace = self.consume_single(TokenKind::LBrace);
                    tokens.push(lbrace);
                    self.consume_expression(&mut tokens)?;
                }
                '}' => {
                    // stray closing brace; parser will validate
                    tokens.push(self.consume_single(TokenKind::RBrace));
                }
                '"' => {
                    let (tok, _) = self.consume_string()?;
                    tokens.push(tok);
                }
                c if c.is_whitespace() => {
                    self.consume_whitespace();
                }
                c if is_ident_start(c) && self.in_tag => {
                    // Identifiers (tag/attr names) only *inside* a tag
                    tokens.push(self.consume_ident());
                }
                _ => {
                    // Everything else outside tags is text
                    let (tok, produced) = self.consume_text();
                    if produced {
                        tokens.push(tok);
                    }
                }
            }
        }

        // EOF
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
        // Update tag state when we see < or >
        match kind {
            TokenKind::LAngle => self.in_tag = true,
            TokenKind::RAngle => self.in_tag = false,
            _ => {}
        }
        let span = self.span_from(mark);
        Token { kind, span }
    }

    fn consume_ident(&mut self) -> Token {
        let mark = self.mark();
        // first char already validated by caller
        self.advance_char();
        while let Some(ch) = self.peek_char() {
            if is_ident_continue(ch) {
                self.advance_char();
            } else {
                break;
            }
        }
        let span = self.span_from(mark);
        let text = &self.input[mark.index..self.index];
        Token {
            kind: TokenKind::Ident(text.to_string()),
            span,
        }
    }

    fn consume_string(&mut self) -> Result<(Token, String), LexError> {
        let mark = self.mark();
        // skip opening "
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
                        None => return Err(LexError::Invalid(self.span_from(mark))),
                    }
                }
                '"' => {
                    // closing "
                    self.advance_char();
                    let span = self.span_from(mark);
                    let tok = Token {
                        kind: TokenKind::Str(value.clone()),
                        span,
                    };
                    return Ok((tok, value));
                }
                _ => {
                    self.advance_char();
                    value.push(ch);
                }
            }
        }

        // EOF before closing quote
        Err(LexError::Invalid(self.span_from(mark)))
    }

    fn consume_text(&mut self) -> (Token, bool) {
        let mark = self.mark();
        let mut value = String::new();

        while let Some(ch) = self.peek_char() {
            // stop at constructs handled by other scanners
            if ch == '<'
                || ch == '{'
                || ch == '}'
                || ch == '"'
                || (self.in_tag && (ch == '/' || ch == '='))
            {
                break;
            }
            // If we encounter an identifier-start *and* we're inside a tag, stop;
            // the tag body (attr names) should be handled by consume_ident.
            if self.in_tag && is_ident_start(ch) {
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
        // assumes we've seen first '/', and peek_next_char() was '/'
        self.advance_char(); // first '/'
        self.advance_char(); // second '/'
        while let Some(ch) = self.peek_char() {
            if ch == '\n' {
                break; // leave newline for main loop to handle col/line
            }
            self.advance_char();
        }
    }

    fn consume_expression(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexError> {
        // we already pushed LBrace
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
        let mut it = self.input[self.index..].chars();
        it.next()?;
        it.next()
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
} // <-- closes impl<'a> Lexer<'a>

#[derive(Copy, Clone, Debug)]
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
    use crate::token::TokenKind;

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
