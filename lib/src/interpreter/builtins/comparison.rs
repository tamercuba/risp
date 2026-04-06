use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

fn eq(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    match args.len() {
        0 => Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
            span,
        }),
        1 => Ok(Value::Bool(true)),
        _ => {
            let first = &args[0].0;
            let all_equal = args[1..].iter().all(|(v, _)| v == first);
            Ok(Value::Bool(all_equal))
        }
    }
}

fn neq(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    match eq(args, span)? {
        Value::Bool(b) => Ok(Value::Bool(!b)),
        _ => unreachable!(),
    }
}

fn num_cmp(a: &Value, b: &Value, span: Span) -> Result<std::cmp::Ordering, RuntimeError> {
    match (a, b) {
        (Value::Long(x), Value::Long(y)) => Ok(x.cmp(y)),
        (Value::Long(x), Value::Double(y)) => Ok((*x as f64).total_cmp(y)),
        (Value::Double(x), Value::Long(y)) => Ok(x.total_cmp(&(*y as f64))),
        (Value::Double(x), Value::Double(y)) => Ok(x.total_cmp(y)),
        (v, _) => Err(RuntimeError::TypeError {
            expected: "number",
            got: v.type_name(),
            span,
        }),
    }
}

fn is_gt(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
            span,
        });
    }
    for pair in args.windows(2) {
        if !num_cmp(&pair[0].0, &pair[1].0, pair[0].1)?.is_gt() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn is_lt(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
            span,
        });
    }
    for pair in args.windows(2) {
        if !num_cmp(&pair[0].0, &pair[1].0, pair[0].1)?.is_lt() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn is_le(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
            span,
        });
    }
    for pair in args.windows(2) {
        if !num_cmp(&pair[0].0, &pair[1].0, pair[0].1)?.is_le() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn is_ge(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
            span,
        });
    }
    for pair in args.windows(2) {
        if !num_cmp(&pair[0].0, &pair[1].0, pair[0].1)?.is_ge() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("=", Value::new_builtin("=", eq)),
        ("not=", Value::new_builtin("not=", neq)),
        (">", Value::new_builtin(">", is_gt)),
        (">=", Value::new_builtin("<=", is_ge)),
        ("<", Value::new_builtin("<", is_lt)),
        ("<=", Value::new_builtin("<=", is_le)),
    ]
}
