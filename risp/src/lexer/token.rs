use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

#[derive(Clone)]
pub struct Content<T> {
    pub content: T,
    pub span: Span,
}

impl<T> Display for Content<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{lo}..{hi} {content}",
            lo = self.span.lo,
            hi = self.span.hi,
            content = format!("{:?}", self.content)
        )
    }
}

impl<T> Debug for Content<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{lo}..{hi} {content}",
            lo = self.span.lo,
            hi = self.span.hi,
            content = format!("{:?}", self.content)
        )
    }
}

impl<T> PartialEq for Content<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl<T> Eq for Content<T> where T: Eq {}

impl Span {
    pub fn at(offset: usize) -> Self {
        Span {
            lo: offset as u32,
            hi: (offset + 1) as u32,
        }
    }
}

impl<T> Content<T> {
    pub fn new(content: T, span: Span) -> Self {
        Content { content, span }
    }
}

#[derive(PartialEq, Clone)]
pub enum Token {
    Integer(Content<i64>),
    Symbol(Content<String>),
    LParen(Content<()>),
    RParen(Content<()>),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(c) => write!(
                f,
                "{lo}..{hi} Int({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::Symbol(c) => write!(
                f,
                "{lo}..{hi} Symbol({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::LParen(c) => write!(f, "{lo}..{hi} LParen", lo = c.span.lo, hi = c.span.hi),
            Token::RParen(c) => write!(f, "{lo}..{hi} RParen", lo = c.span.lo, hi = c.span.hi),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(c) => write!(
                f,
                "{lo}..{hi} Int({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::Symbol(c) => write!(
                f,
                "{lo}..{hi} Symbol({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::LParen(c) => write!(f, "{lo}..{hi} LParen", lo = c.span.lo, hi = c.span.hi),
            Token::RParen(c) => write!(f, "{lo}..{hi} RParen", lo = c.span.lo, hi = c.span.hi),
        }
    }
}
