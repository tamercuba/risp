#![allow(dead_code)]

use crate::lexer::token::Content;
use crate::lexer::{Span, Token};
use crate::parser::cst::{Expr, ExprKind};

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

#[derive(Debug)]
pub enum Frame {
    List(Vec<Expr>, Span),
    Vector(Vec<Expr>, Span),
    Map(Vec<Expr>, Span),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    stack: Vec<Frame>,
    result: Vec<Expr>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().rev().collect::<Vec<Token>>(),
            stack: vec![],
            result: vec![],
        }
    }

    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, ParseError> {
        let parser = Self::new(tokens);
        parser.run()
    }

    fn run(mut self) -> Result<Vec<Expr>, ParseError> {
        while let Some(token) = self.tokens.pop() {
            match token {
                Token::LParen(c) => self.stack.push(Frame::List(vec![], c.span)),
                Token::RParen(c) => self.parse_r_paren(c.span)?,
                Token::LBracket(c) => self.stack.push(Frame::Vector(vec![], c.span)),
                Token::RBracket(c) => self.parse_r_bracket(c.span)?,
                Token::LBrace(c) => self.stack.push(Frame::Map(vec![], c.span)),
                Token::RBrace(c) => self.parse_r_brace(c.span)?,
                Token::Long(c) => self.push_to_frame(ExprKind::Long(c.content), c.span)?,
                Token::Double(c) => self.push_to_frame(ExprKind::Double(c.content), c.span)?,
                Token::Symbol(c) => self.parse_symbol(c)?,
                Token::String(c) => self.push_to_frame(ExprKind::String(c.content), c.span)?,
                Token::Keyword(c) => self.push_to_frame(ExprKind::Keyword(c.content), c.span)?,
            }
        }

        if let Some(frame) = self.stack.last() {
            let span = match frame {
                Frame::List(_, s) | Frame::Vector(_, s) | Frame::Map(_, s) => s.clone(),
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
        match self.stack.last_mut() {
            Some(Frame::List(elems, _)) => elems.push(expr),
            Some(Frame::Vector(elems, _)) => elems.push(expr),
            Some(Frame::Map(elems, _)) => elems.push(expr),
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
            None => Err(ParseError::UnmatchedClose('}', current_span)),
        }
    }
}
