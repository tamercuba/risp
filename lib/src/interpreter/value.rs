use crate::collections::RispList;
use crate::lexer::Span;
use crate::sema::{AstNode, FnArity};
use std::{cell::RefCell, rc::Rc};

use super::env::Env;

type BuiltinFn = fn(&[(Value, Span)], Span) -> Result<Value, RuntimeError>;

#[derive(Clone)]
pub struct ClosureArity {
    pub params: Vec<u32>,
    pub variadic: Option<u32>,
    pub body: Rc<AstNode>,
}

impl From<&FnArity> for ClosureArity {
    fn from(a: &FnArity) -> Self {
        Self {
            params: a.params.clone(),
            variadic: a.variadic,
            body: a.body.clone(),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable {
        name: String,
        span: Span,
    },
    NotCallable {
        span: Span,
    },
    WrongArity {
        expected: usize,
        got: usize,
        span: Span,
    },
    TypeError {
        expected: &'static str,
        got: &'static str,
        span: Span,
    },
    UnsupportedType {
        t: &'static str,
        span: Span,
    },
    IndexOutOfBounds {
        max_accessible: usize,
        got: usize,
        span: Span,
    },
    DivisionByZero(Span),
    ParseError(String),
    AnalyzeError(String),
    RecurOutsideLoop {
        span: Span,
    },
}

#[derive(Clone)]
pub enum EvalFlow {
    Value(Value),
    Recur(Vec<Value>),
}

#[derive(Clone)]
pub enum Callable {
    Closure {
        arities: Vec<ClosureArity>,
        env: Rc<RefCell<Env>>,
    },
    Builtin {
        name: &'static str,
        func: BuiltinFn,
    },
}

#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Long(i64),
    Double(f64),
    String(String),
    Keyword(String),
    List(RispList<Value>),
    Vector(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Set(Vec<Value>),
    Symbol(String),
    Callable(Rc<Callable>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Long(a), Value::Long(b)) => a == b,
            (Value::Double(a), Value::Double(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Keyword(a), Value::Keyword(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::List(a), Value::List(b)) => a == b,
            (Value::Vector(a), Value::Vector(b)) => a == b,
            (Value::List(l), Value::Vector(v)) | (Value::Vector(v), Value::List(l)) => {
                l.len() == v.len() && l.iter().zip(v.iter()).all(|(x, y)| x == y)
            }
            (Value::Set(a), Value::Set(b)) => a == b,
            (Value::Map(a), Value::Map(b)) => a == b,
            // Callables are never equal
            _ => false,
        }
    }
}
impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::Long(n) => write!(f, "Long({n})"),
            Value::Double(n) => write!(f, "Double({n})"),
            Value::String(s) => write!(f, "String({s:?})"),
            Value::Keyword(s) => write!(f, "Keyword({s})"),
            Value::List(v) => write!(f, "List({v:?})"),
            Value::Vector(v) => write!(f, "Vector({v:?})"),
            Value::Map(m) => write!(f, "Map({m:?})"),
            Value::Set(v) => write!(f, "Set({v:?})"),
            Value::Symbol(s) => write!(f, "Symbol({s})"),
            Value::Callable(_) => write!(f, "Callable(...)"),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable { name, .. } => {
                write!(f, "(undefined-variable '{name})")
            }
            RuntimeError::NotCallable { .. } => write!(f, "(not-callable)"),
            RuntimeError::WrongArity { expected, got, .. } => {
                write!(
                    f,
                    "(wrong-number-of-args\n  (expected {expected})\n  (got {got}))"
                )
            }
            RuntimeError::TypeError { expected, got, .. } => {
                write!(f, "(type-error\n  (expected {expected})\n  (got {got}))")
            }
            RuntimeError::DivisionByZero(_) => write!(f, "(division-by-zero)"),
            RuntimeError::ParseError(msg) => write!(f, "(parse-error\n  {msg})"),
            RuntimeError::AnalyzeError(msg) => write!(f, "(analyze-error {msg})"),
            RuntimeError::UnsupportedType { t, span: _ } => write!(f, "(unsupported-type \"{t})\""),
            RuntimeError::IndexOutOfBounds {
                max_accessible,
                got,
                span: _,
            } => write!(
                f,
                "(index-out-of-bounds\n  (max-index {max_accessible})\n  (got {got}))",
            ),
            RuntimeError::RecurOutsideLoop { .. } => write!(f, "(recur-outside-loop)"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Long(n) => write!(f, "{n}"),
            Value::Double(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Keyword(s) => write!(f, ":{s}"),
            Value::List(v) => write!(f, "{v}"),
            Value::Vector(v) => {
                write!(f, "[")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{e}")?;
                }
                write!(f, "]")
            }
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k} {v}")?;
                }
                write!(f, "}}")
            }
            Value::Set(v) => {
                write!(f, "#{{")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{e}")?;
                }
                write!(f, "}}")
            }
            Value::Symbol(s) => write!(f, "{s}"),
            Value::Callable(_) => write!(f, "#<fn>"),
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Long(_) => "long",
            Value::Double(_) => "double",
            Value::String(_) => "string",
            Value::Keyword(_) => "keyword",
            Value::List(_) => "list",
            Value::Vector(_) => "vector",
            Value::Map(_) => "map",
            Value::Set(_) => "set",
            Value::Symbol(_) => "symbol",
            Value::Callable(_) => "callable",
        }
    }

    pub fn new_builtin(name: &'static str, func: BuiltinFn) -> Value {
        Value::Callable(Rc::new(Callable::Builtin { name, func }))
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }
}
