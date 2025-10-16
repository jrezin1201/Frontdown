use crate::ast::{Attr, AttrValue, Document, Node};
use crate::token::Span;
use crate::token::{Token, TokenKind};
use thiserror::Error;

pub fn parse(tokens: &[Token], src: &str) -> Result<Document, ParseError> {
    let mut parser = Parser::new(tokens, src);
    parser.parse_document()
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unexpected token {found:?} at {span:?}, expected {expected}")]
    Unexpected {
        found: TokenKind,
        span: Span,
        expected: &'static str,
    },
    #[error("unclosed tag <{0}> at {1:?}")]
    UnclosedTag(String, Span),
    #[error("mismatched closing tag: expected </{expected}>, got </{found}> at {span:?}")]
    MismatchedClose {
        expected: String,
        found: String,
        span: Span,
    },
}

struct Parser<'a> {
    tokens: &'a [Token],
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token], src: &'a str) -> Self {
        Self {
            tokens,
            src,
            pos: 0,
        }
    }

    fn parse_document(&mut self) -> Result<Document, ParseError> {
        let mut children = Vec::new();
        while !self.current_is(TokenKind::Eof) {
            children.push(self.parse_node()?);
        }
        Ok(Document { children })
    }

    fn parse_node(&mut self) -> Result<Node, ParseError> {
        match self.current().kind.clone() {
            TokenKind::LAngle => self.parse_element(),
            TokenKind::Text(text) => {
                self.pos += 1;
                Ok(Node::Text(text))
            }
            TokenKind::LBrace => {
                let expr = self.parse_expr_string()?;
                Ok(Node::Expr(expr))
            }
            _ => Err(self.unexpected(&self.current().clone(), "element, text, or expression")),
        }
    }

    fn parse_element(&mut self) -> Result<Node, ParseError> {
        let open = self.expect(TokenKind::LAngle, "<")?;
        let name_token = self.expect_ident("tag name")?;
        let name = match &name_token.kind {
            TokenKind::Ident(value) => value.clone(),
            _ => unreachable!(),
        };
        let mut attrs = Vec::new();
        loop {
            match self.current().kind.clone() {
                TokenKind::Ident(_) => {
                    attrs.push(self.parse_attr()?);
                }
                _ => break,
            }
        }

        match self.current().kind.clone() {
            TokenKind::Slash => {
                self.pos += 1;
                self.expect(TokenKind::RAngle, ">")?;
                Ok(Node::Element {
                    name,
                    attrs,
                    children: Vec::new(),
                })
            }
            TokenKind::RAngle => {
                self.pos += 1;
                let mut children = Vec::new();
                while !self.next_is_closing_tag() {
                    if self.current_is(TokenKind::Eof) {
                        return Err(ParseError::UnclosedTag(name.clone(), open.span));
                    }
                    children.push(self.parse_node()?);
                }
                self.expect(TokenKind::LAngle, "<")?;
                self.expect(TokenKind::Slash, "/")?;
                let closing_name = self.expect_ident("closing tag name")?;
                let found_name = match closing_name.kind {
                    TokenKind::Ident(ref value) => value.clone(),
                    _ => unreachable!(),
                };
                if found_name != name {
                    return Err(ParseError::MismatchedClose {
                        expected: name,
                        found: found_name,
                        span: closing_name.span,
                    });
                }
                self.expect(TokenKind::RAngle, ">")?;
                Ok(Node::Element {
                    name,
                    attrs,
                    children,
                })
            }
            _ => Err(self.unexpected(&self.current().clone(), "`>` or `/>`")),
        }
    }

    fn parse_attr(&mut self) -> Result<Attr, ParseError> {
        let name_token = self.expect_ident("attribute name")?;
        let name = if let TokenKind::Ident(value) = name_token.kind.clone() {
            value
        } else {
            unreachable!()
        };
        self.expect(TokenKind::Equals, "=")?;
        let value = match self.current().kind.clone() {
            TokenKind::Str(s) => {
                self.pos += 1;
                AttrValue::Str(s)
            }
            TokenKind::LBrace => {
                let expr = self.parse_expr_string()?;
                AttrValue::Expr(expr)
            }
            _ => {
                return Err(self.unexpected(&self.current().clone(), "string literal or expression"))
            }
        };
        Ok(Attr { name, value })
    }

    fn parse_expr_string(&mut self) -> Result<String, ParseError> {
        let lbrace = self.expect(TokenKind::LBrace, "{")?;
        let start = lbrace.span.end_offset();
        let rbrace = self.expect(TokenKind::RBrace, "}")?;
        let end = rbrace.span.offset;
        let slice = self
            .src
            .get(start..end)
            .ok_or_else(|| ParseError::Unexpected {
                found: TokenKind::RBrace,
                span: rbrace.span.clone(),
                expected: "expression contents",
            })?;
        Ok(slice.trim().to_string())
    }

    fn expect(
        &mut self,
        expected: TokenKind,
        description: &'static str,
    ) -> Result<Token, ParseError> {
        let token = self.current().clone();
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            self.pos += 1;
            Ok(token)
        } else {
            Err(self.unexpected(&token, description))
        }
    }

    fn expect_ident(&mut self, description: &'static str) -> Result<Token, ParseError> {
        let token = self.current().clone();
        if matches!(token.kind, TokenKind::Ident(_)) {
            self.pos += 1;
            Ok(token)
        } else {
            Err(self.unexpected(&token, description))
        }
    }

    fn next_is_closing_tag(&self) -> bool {
        matches!(
            self.tokens.get(self.pos),
            Some(Token {
                kind: TokenKind::LAngle,
                ..
            })
        ) && matches!(
            self.tokens.get(self.pos + 1),
            Some(Token {
                kind: TokenKind::Slash,
                ..
            })
        )
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(&kind)
    }

    fn unexpected(&self, token: &Token, expected: &'static str) -> ParseError {
        ParseError::Unexpected {
            found: token.kind.clone(),
            span: token.span.clone(),
            expected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn parses_simple_element() {
        let src = "<div>Hello</div>";
        let tokens = lex(src).unwrap();
        let doc = parse(&tokens, src).unwrap();
        assert_eq!(doc.children.len(), 1);
        match &doc.children[0] {
            Node::Element { name, children, .. } => {
                assert_eq!(name, "div");
                assert_eq!(children.len(), 1);
            }
            _ => panic!("expected element"),
        }
    }
}
