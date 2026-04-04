#![allow(dead_code)]

use crate::lexer::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Long(i64),
    Double(f64),
    Bool(bool),
    Nil,
    String(String),
    Keyword(String),
    Symbol(String),

    List(Vec<Expr>),
    Vector(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Set(Vec<Expr>),
    Quote(Box<Expr>),
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Expr { kind, span }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
