use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

//(cons 0 '(1 2))
//(concat [1 2] [3 4])
//(seq [])

fn count(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    match elems.len() {
        1 => match elems[0].0.clone() {
            Value::List(c) | Value::Vector(c) | Value::Set(c) => Ok(Value::Long(c.len() as i64)),
            Value::Map(c) => Ok(Value::Long(c.len() as i64)),
            v => Err(RuntimeError::TypeError {
                expected: "seq",
                got: v.type_name(),
                span,
            }),
        },
        _ => Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        }),
    }
}

fn first(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    match elems.len() {
        1 => match (elems[0].0.clone(), elems[0].1.clone()) {
            (Value::List(c), _) | (Value::Set(c), _) | (Value::Vector(c), _) => match c.first() {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Nil),
            },
            (Value::Map(m), _) => match m.first() {
                Some(f) => Ok(Value::Vector(vec![f.0.clone(), f.1.clone()])),
                None => Ok(Value::Nil),
            },
            (value, span) => Err(RuntimeError::TypeError {
                expected: "seq",
                got: value.type_name(),
                span,
            }),
        },
        _ => Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        }),
    }
}

fn rest(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if elems.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        });
    }

    let col = elems.first().unwrap();

    match col {
        (Value::List(c), _) | (Value::Set(c), _) | (Value::Vector(c), _) => {
            if !c.is_empty() {
                Ok(Value::List(c[1..].to_vec()))
            } else {
                Ok(Value::List([].to_vec()))
            }
        }
        (Value::Map(m), _) => {
            if !m.is_empty() {
                let result = m[1..]
                    .iter()
                    .map(|(k, v)| Value::Vector(vec![k.clone(), v.clone()]))
                    .collect::<Vec<Value>>();
                Ok(Value::List(result))
            } else {
                Ok(Value::List(vec![]))
            }
        }
        (v, s) => Err(RuntimeError::TypeError {
            expected: "seq",
            got: v.type_name(),
            span: s.clone(),
        }),
    }
}

fn second(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if elems.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        });
    }

    let col = elems.first().unwrap();

    match col {
        (Value::List(c), _) | (Value::Set(c), _) | (Value::Vector(c), _) => {
            if c.len() < 2 {
                Ok(Value::Nil)
            } else {
                Ok(c[1].clone())
            }
        }
        (Value::Map(m), _) => {
            if m.len() < 2 {
                Ok(Value::Nil)
            } else {
                let (k, v) = m[2].clone();
                Ok(Value::Vector(vec![k, v]))
            }
        }
        (v, s) => Err(RuntimeError::TypeError {
            expected: "seq",
            got: v.type_name(),
            span: s.clone(),
        }),
    }
}

fn last(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    match elems.len() {
        1 => match (elems[0].0.clone(), elems[0].1.clone()) {
            (Value::List(c), _) | (Value::Set(c), _) | (Value::Vector(c), _) => match c.last() {
                Some(v) => Ok(v.clone()),
                None => Ok(Value::Nil),
            },
            (Value::Map(m), _) => match m.last() {
                Some(f) => Ok(Value::Vector(vec![f.0.clone(), f.1.clone()])),
                None => Ok(Value::Nil),
            },
            (value, span) => Err(RuntimeError::TypeError {
                expected: "seq",
                got: value.type_name(),
                span,
            }),
        },
        _ => Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        }),
    }
}

fn nth(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if elems.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: elems.len(),
            span,
        });
    }

    let (col, col_span) = elems[0].clone();
    let (n_value, n_span) = elems[1].clone();

    match (col.clone(), n_value) {
        (_, Value::Long(n)) if n < 0 => Err(RuntimeError::TypeError {
            expected: "non-negative index",
            got: "negative long",
            span: n_span,
        }),
        (Value::List(c), Value::Long(n))
        | (Value::Set(c), Value::Long(n))
        | (Value::Vector(c), Value::Long(n)) => match c.get(n as usize) {
            Some(v) => Ok(v.clone()),
            None => Err(RuntimeError::IndexOutOfBounds {
                max_accessible: c.len() - 1,
                got: n as usize,
                span: col_span.clone(),
            }),
        },
        (Value::Map(_), _) => Err(RuntimeError::UnsupportedType {
            t: col.type_name(),
            span: col_span.clone(),
        }),
        (value, _) => Err(RuntimeError::UnsupportedType {
            t: value.type_name(),
            span: col_span.clone(),
        }),
    }
}

fn conj(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if elems.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: elems.len(),
            span,
        });
    }

    let (col, col_span) = &elems[0];
    let (val, val_span) = &elems[1];

    match col {
        Value::Vector(v) => {
            let mut result = v.clone();
            result.push(val.clone());
            Ok(Value::Vector(result))
        }
        Value::List(l) => {
            let mut result = vec![val.clone()];
            result.extend(l.clone());
            Ok(Value::List(result))
        }
        Value::Set(s) => {
            let mut result = s.clone();
            if !result.contains(val) {
                result.push(val.clone());
            }
            Ok(Value::Set(result))
        }
        Value::Map(m) => {
            let pair = match val {
                Value::Vector(v) if v.len() == 2 => Ok((v[0].clone(), v[1].clone())),
                Value::Map(other) => {
                    let mut result = m.clone();
                    for (k, v) in other {
                        if let Some(entry) = result.iter_mut().find(|(ek, _)| ek == k) {
                            entry.1 = v.clone();
                        } else {
                            result.push((k.clone(), v.clone()));
                        }
                    }
                    return Ok(Value::Map(result));
                }
                v => Err(RuntimeError::TypeError {
                    expected: "vector pair or map",
                    got: v.type_name(),
                    span: val_span.clone(),
                }),
            }?;

            let mut result = m.clone();
            if let Some(entry) = result.iter_mut().find(|(k, _)| k == &pair.0) {
                entry.1 = pair.1;
            } else {
                result.push(pair);
            }
            Ok(Value::Map(result))
        }
        v => Err(RuntimeError::TypeError {
            expected: "seq",
            got: v.type_name(),
            span: col_span.clone(),
        }),
    }
}

fn empty(elems: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if elems.len() != 1 {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: elems.len(),
            span,
        });
    }

    match elems.first().unwrap().clone() {
        (Value::List(c), _) | (Value::Vector(c), _) | (Value::Set(c), _) => {
            Ok(Value::Bool(c.is_empty()))
        }
        (Value::Map(m), _) => Ok(Value::Bool(m.is_empty())),
        (v, s) => Err(RuntimeError::TypeError {
            expected: "collection",
            got: v.type_name(),
            span: s.clone(),
        }),
    }
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("count", Value::new_builtin("count", count)),
        ("first", Value::new_builtin("first", first)),
        ("rest", Value::new_builtin("rest", rest)),
        ("second", Value::new_builtin("second", second)),
        ("last", Value::new_builtin("last", last)),
        ("nth", Value::new_builtin("nth", nth)),
        ("conj", Value::new_builtin("conj", conj)),
        ("empty?", Value::new_builtin("empty?", empty)),
    ]
}
