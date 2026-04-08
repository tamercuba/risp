use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

fn write(args: &[(Value, Span)], span: Span) -> Result<Value, RuntimeError> {
    if args.len() > 1 || args.is_empty() {
        return Err(RuntimeError::WrongArity {
            expected: 1,
            got: args.len(),
            span,
        });
    }
    match &args[0] {
        (Value::String(s), _) => {
            print!("{}", s);
            Ok(Value::Nil)
        }
        (v, s) => Err(RuntimeError::UnsupportedType {
            t: v.type_name().to_string(),
            span: *s,
        }),
    }
}

fn str_conv(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    Ok(Value::String(
        args.iter().map(|(v, _)| v.to_string()).collect(),
    ))
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("write", Value::new_builtin("write", write)),
        ("str", Value::new_builtin("str", str_conv)),
    ]
}
