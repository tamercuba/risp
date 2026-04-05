use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

impl Span {
    pub fn full(&self, other: Self) -> Self {
        Self {
            lo: self.lo,
            hi: other.hi,
        }
    }
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
        write!(f, "{}..{} {:?}", self.span.lo, self.span.hi, self.content)
    }
}

impl<T> Debug for Content<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{} {:?}", self.span.lo, self.span.hi, self.content)
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
    Long(Content<i64>),
    Double(Content<f64>),
    Symbol(Content<String>),
    String(Content<String>),
    Keyword(Content<String>),
    LParen(Content<()>),
    RParen(Content<()>),
    LBracket(Content<()>),
    RBracket(Content<()>),
    LBrace(Content<()>),
    RBrace(Content<()>),
    Hash(Content<()>),
    Quote(Content<()>),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Long(c) => write!(
                f,
                "{lo}..{hi} Long({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::Double(c) => write!(
                f,
                "{lo}..{hi} Double({value})",
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
            Token::Keyword(c) => write!(
                f,
                "{lo}..{hi} Keyword({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::String(c) => write!(
                f,
                "{lo}..{hi} String({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),

            Token::LBracket(c) => write!(f, "{lo}..{hi} LBracket", lo = c.span.lo, hi = c.span.hi),
            Token::RBracket(c) => write!(f, "{lo}..{hi} RBracket", lo = c.span.lo, hi = c.span.hi),
            Token::LBrace(c) => write!(f, "{lo}..{hi} LBrace", lo = c.span.lo, hi = c.span.hi),
            Token::RBrace(c) => write!(f, "{lo}..{hi} RBrace", lo = c.span.lo, hi = c.span.hi),
            Token::Hash(c) => write!(f, "{lo}..{hi} Hash", lo = c.span.lo, hi = c.span.hi),
            Token::Quote(c) => write!(f, "{lo}..{hi} Quote", lo = c.span.lo, hi = c.span.hi),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Long(c) => write!(
                f,
                "{lo}..{hi} Long({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::Double(c) => write!(
                f,
                "{lo}..{hi} Double({value})",
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
            Token::Keyword(c) => write!(
                f,
                "{lo}..{hi} Keyword({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::String(c) => write!(
                f,
                "{lo}..{hi} String({value})",
                lo = c.span.lo,
                hi = c.span.hi,
                value = c.content
            ),
            Token::LBracket(c) => write!(f, "{lo}..{hi} LBracket", lo = c.span.lo, hi = c.span.hi),
            Token::RBracket(c) => write!(f, "{lo}..{hi} RBracket", lo = c.span.lo, hi = c.span.hi),
            Token::LBrace(c) => write!(f, "{lo}..{hi} LBrace", lo = c.span.lo, hi = c.span.hi),
            Token::RBrace(c) => write!(f, "{lo}..{hi} RBrace", lo = c.span.lo, hi = c.span.hi),
            Token::Hash(c) => write!(f, "{lo}..{hi} Hash", lo = c.span.lo, hi = c.span.hi),
            Token::Quote(c) => write!(f, "{lo}..{hi} Quote", lo = c.span.lo, hi = c.span.hi),
        }
    }
}
