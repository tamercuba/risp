use std::rc::Rc;

use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

fn list(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    Ok(Value::List(args.iter().map(|t| t.0.clone()).collect()))
}

fn vec(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    Ok(Value::Vector(Rc::new(
        args.iter().map(|t| t.0.clone()).collect::<Vec<Value>>(),
    )))
}

fn map(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    Ok(Value::Map(Rc::new(
        args.chunks(2)
            .map(|pair| (pair[0].0.clone(), pair[1].0.clone()))
            .collect::<Vec<(Value, Value)>>(),
    )))
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("list", Value::new_builtin("list", list)),
        ("vector", Value::new_builtin("vector", vec)),
        ("hash-map", Value::new_builtin("hash-map", map)),
    ]
}
