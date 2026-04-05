use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

fn sum(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    let mut has_float = false;
    let mut result_int: i64 = 0;
    let mut result_float: f64 = 0.0;

    for (arg, span) in args {
        match arg {
            Value::Long(v) => {
                if has_float {
                    result_float += *v as f64;
                } else {
                    result_int += *v;
                }
                Ok(())
            }
            Value::Double(v) => {
                if !has_float {
                    has_float = true;
                    result_float = result_int as f64;
                }
                result_float += *v;
                Ok(())
            }
            w => Err(RuntimeError::TypeError {
                expected: "long or double",
                got: w.type_name(),
                span: span.clone(),
            }),
        }?;
    }

    if has_float {
        Ok(Value::Double(result_float))
    } else {
        Ok(Value::Long(result_int))
    }
}

fn minus(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: 0,
            span,
        });
    }

    let (first, first_span) = &args[0];
    let (mut has_float, mut result_int, mut result_float) = match first {
        Value::Long(v) => (false, *v, 0.0f64),
        Value::Double(v) => (true, 0i64, *v),
        w => {
            return Err(RuntimeError::TypeError {
                expected: "long or double",
                got: w.type_name(),
                span: first_span.clone(),
            })
        }
    };

    if args.len() == 1 {
        return if has_float {
            Ok(Value::Double(-result_float))
        } else {
            Ok(Value::Long(-result_int))
        };
    }

    for (arg, span) in &args[1..] {
        match arg {
            Value::Long(v) => {
                if has_float {
                    result_float -= *v as f64;
                } else {
                    result_int -= *v;
                }
                Ok(())
            }
            Value::Double(v) => {
                if !has_float {
                    has_float = true;
                    result_float = result_int as f64;
                }
                result_float -= *v;
                Ok(())
            }
            w => Err(RuntimeError::TypeError {
                expected: "long or double",
                got: w.type_name(),
                span: span.clone(),
            }),
        }?;
    }

    if has_float {
        Ok(Value::Double(result_float))
    } else {
        Ok(Value::Long(result_int))
    }
}

fn times(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    let mut has_float = false;
    let mut result_int: i64 = 1;
    let mut result_float: f64 = 1.0;

    for (arg, span) in args {
        match arg {
            Value::Long(v) => {
                if has_float {
                    result_float *= *v as f64;
                } else {
                    result_int *= *v;
                }
                Ok(())
            }
            Value::Double(v) => {
                if !has_float {
                    has_float = true;
                    result_float = result_int as f64;
                }
                result_float *= *v;
                Ok(())
            }
            w => Err(RuntimeError::TypeError {
                expected: "long or double",
                got: w.type_name(),
                span: span.clone(),
            }),
        }?;
    }

    if has_float {
        Ok(Value::Double(result_float))
    } else {
        Ok(Value::Long(result_int))
    }
}

fn divide(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::WrongArity {
            expected: 2,
            got: args.len(),
            span,
        });
    }

    let (dividend, dividend_span) = args[0].clone();
    let (divider, divider_span) = args[1].clone();

    match (dividend, divider) {
        (_, Value::Long(0)) => Err(RuntimeError::DivisionByZero(divider_span)),
        (_, Value::Double(0.0)) => Err(RuntimeError::DivisionByZero(divider_span)),
        (Value::Long(a), Value::Long(b)) => Ok(Value::Long(a / b)),
        (Value::Long(a), Value::Double(b)) => Ok(Value::Double(a as f64 / b)),
        (Value::Double(a), Value::Long(b)) => Ok(Value::Double(a / b as f64)),
        (Value::Double(a), Value::Double(b)) => Ok(Value::Double(a / b)),
        (v, _) => Err(RuntimeError::TypeError {
            expected: "long or double",
            got: v.type_name(),
            span: dividend_span,
        }),
    }
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("+", Value::new_builtin("+", sum)),
        ("-", Value::new_builtin("-", minus)),
        ("*", Value::new_builtin("*", times)),
        ("/", Value::new_builtin("/", divide)),
    ]
}
