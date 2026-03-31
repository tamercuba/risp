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

    List(Vec<ExprKind>),
    Vector(Vec<ExprKind>),
    Map(Vec<(ExprKind, ExprKind)>),
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
