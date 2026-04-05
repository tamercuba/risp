mod cst;
#[cfg(test)]
mod test_parser;

use crate::lexer::{Content, Span, Token};
pub use crate::parser::cst::{Expr, ExprKind};

#[derive(Debug)]
pub enum ParseError {
    UnmatchedOpen(Span),
    UnmatchedClose(char, Span),
    MismatchedDelimiter {
        expected: char,
        found: char,
        span: Span,
    },
    OddMapElements(Span),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnmatchedOpen(span) => {
                write!(f, "(unmatched-open :at {})", span.lo)
            }
            ParseError::UnmatchedClose(ch, span) => {
                write!(f, "(unmatched-close :char '{ch}' :at {})", span.lo)
            }
            ParseError::MismatchedDelimiter { expected, found, span } => write!(
                f,
                "(mismatched-delimiter :expected '{expected}' :found '{found}' :at {})",
                span.lo
            ),
            ParseError::OddMapElements(span) => {
                write!(f, "(odd-map-elements :at {})", span.lo)
            }
        }
    }
}

#[derive(Debug)]
pub enum Frame {
    List(Vec<Expr>, Span),
    Vector(Vec<Expr>, Span),
    Map(Vec<Expr>, Span),
    Set(Vec<Expr>, Span),
    Quote(Span),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    stack: Vec<Frame>,
    result: Vec<Expr>,
    pending_hash: bool,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect::<Vec<Token>>(),
            stack: vec![],
            result: vec![],
            pending_hash: false,
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParseError> {
        let parser = Self::new(tokens);
        parser.run()
    }

    fn run(mut self) -> Result<Vec<Expr>, ParseError> {
        while let Some(token) = self.tokens.pop() {
            match token {
                Token::LParen(c) => {
                    self.pending_hash = false;
                    self.stack.push(Frame::List(vec![], c.span));
                }
                Token::RParen(c) => self.parse_r_paren(c.span)?,
                Token::LBracket(c) => {
                    self.pending_hash = false;
                    self.stack.push(Frame::Vector(vec![], c.span));
                }
                Token::RBracket(c) => self.parse_r_bracket(c.span)?,
                Token::LBrace(c) => {
                    if self.pending_hash {
                        self.pending_hash = false;
                        self.stack.push(Frame::Set(vec![], c.span));
                    } else {
                        self.stack.push(Frame::Map(vec![], c.span));
                    }
                }
                Token::RBrace(c) => self.parse_r_brace(c.span)?,
                Token::Hash(c) => {
                    self.pending_hash = true;
                    // consume the span; actual frame pushed when LBrace arrives
                    let _ = c;
                }
                Token::Quote(c) => {
                    self.stack.push(Frame::Quote(c.span));
                }
                Token::Long(c) => self.push_to_frame(ExprKind::Long(c.content), c.span)?,
                Token::Double(c) => self.push_to_frame(ExprKind::Double(c.content), c.span)?,
                Token::Symbol(c) => self.parse_symbol(c)?,
                Token::String(c) => self.push_to_frame(ExprKind::String(c.content), c.span)?,
                Token::Keyword(c) => self.push_to_frame(ExprKind::Keyword(c.content), c.span)?,
            }
        }

        if let Some(frame) = self.stack.last() {
            let span = match frame {
                Frame::List(_, s) | Frame::Vector(_, s) | Frame::Map(_, s) | Frame::Set(_, s) => {
                    s.clone()
                }
                Frame::Quote(s) => s.clone(),
            };
            return Err(ParseError::UnmatchedOpen(span));
        }

        Ok(self.result)
    }

    fn parse_symbol(&mut self, c: Content<String>) -> Result<(), ParseError> {
        let kind = match c.content.as_str() {
            "true" => ExprKind::Bool(true),
            "false" => ExprKind::Bool(false),
            "nil" => ExprKind::Nil,
            _ => ExprKind::Symbol(c.content),
        };
        self.push_to_frame(kind, c.span)
    }

    fn push_to_frame(&mut self, expr_kind: ExprKind, span: Span) -> Result<(), ParseError> {
        let expr = Expr {
            kind: expr_kind,
            span,
        };
        self.push_expr(expr)
    }

    fn push_expr(&mut self, expr: Expr) -> Result<(), ParseError> {
        // Check if the top frame is a Quote — if so, close it immediately
        if matches!(self.stack.last(), Some(Frame::Quote(_))) {
            let frame = self.stack.pop().unwrap();
            let quote_span = match frame {
                Frame::Quote(s) => s,
                _ => unreachable!(),
            };
            let full_span = quote_span.full(expr.span.clone());
            let quoted = Expr {
                kind: ExprKind::Quote(Box::new(expr)),
                span: full_span,
            };
            return self.push_expr(quoted);
        }

        match self.stack.last_mut() {
            Some(Frame::List(elems, _)) => elems.push(expr),
            Some(Frame::Vector(elems, _)) => elems.push(expr),
            Some(Frame::Map(elems, _)) => elems.push(expr),
            Some(Frame::Set(elems, _)) => elems.push(expr),
            Some(Frame::Quote(_)) => unreachable!(),
            None => self.result.push(expr),
        }
        Ok(())
    }

    fn parse_r_paren(&mut self, current_span: Span) -> Result<(), ParseError> {
        match self.stack.pop() {
            Some(Frame::List(elems, span)) => {
                self.push_to_frame(ExprKind::List(elems), span.full(current_span))
            }
            Some(Frame::Vector(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: ']',
                found: ')',
                span,
            }),
            Some(Frame::Map(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: '}',
                found: ')',
                span,
            }),
            Some(Frame::Set(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: '}',
                found: ')',
                span,
            }),
            Some(Frame::Quote(span)) => Err(ParseError::UnmatchedOpen(span)),
            None => Err(ParseError::UnmatchedClose(')', current_span)),
        }
    }

    fn parse_r_bracket(&mut self, current_span: Span) -> Result<(), ParseError> {
        match self.stack.pop() {
            Some(Frame::Vector(elems, span)) => {
                self.push_to_frame(ExprKind::Vector(elems), span.full(current_span))
            }
            Some(Frame::List(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: ')',
                found: ']',
                span,
            }),
            Some(Frame::Map(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: '}',
                found: ']',
                span,
            }),
            Some(Frame::Set(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: '}',
                found: ']',
                span,
            }),
            Some(Frame::Quote(span)) => Err(ParseError::UnmatchedOpen(span)),
            None => Err(ParseError::UnmatchedClose(']', current_span)),
        }
    }

    fn parse_r_brace(&mut self, current_span: Span) -> Result<(), ParseError> {
        match self.stack.pop() {
            Some(Frame::Map(elems, span)) => {
                if elems.len() % 2 != 0 {
                    return Err(ParseError::OddMapElements(current_span));
                }
                let map_elemen = elems
                    .chunks(2)
                    .map(|pair| (pair[0].clone(), pair[1].clone()))
                    .collect::<Vec<(Expr, Expr)>>();

                self.push_to_frame(ExprKind::Map(map_elemen), span.full(current_span))
            }
            Some(Frame::Set(elems, span)) => {
                self.push_to_frame(ExprKind::Set(elems), span.full(current_span))
            }
            Some(Frame::List(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: ')',
                found: '}',
                span,
            }),
            Some(Frame::Vector(_, span)) => Err(ParseError::MismatchedDelimiter {
                expected: ']',
                found: '}',
                span,
            }),
            Some(Frame::Quote(span)) => Err(ParseError::UnmatchedOpen(span)),
            None => Err(ParseError::UnmatchedClose('}', current_span)),
        }
    }
}
