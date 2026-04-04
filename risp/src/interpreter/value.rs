#![allow(dead_code)]
use crate::lexer::Span;
use crate::sema::node::AstNode;
use std::{cell::RefCell, rc::Rc};

use super::env::Env;

type BuiltinFn = Rc<dyn Fn(&[Value]) -> Result<Value, RuntimeError>>;

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
    DivisionByZero(Span),
}

#[derive(Clone)]
pub enum Callable {
    Closure {
        params: Vec<u32>,
        body: Box<AstNode>,
        env: Rc<RefCell<Env>>,
    },
    Builtin {
        name: &'static str,
        func: fn(&[Value]) -> Result<Value, RuntimeError>,
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
    List(Vec<Value>),
    Vector(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Callable(Rc<Callable>),
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
            Value::Callable(_) => write!(f, "Callable(...)"),
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
            Value::Callable(_) => "callable",
        }
    }
}
