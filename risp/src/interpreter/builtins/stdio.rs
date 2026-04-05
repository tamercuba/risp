use crate::interpreter::{RuntimeError, Value};
use crate::lexer::Span;

fn print_line(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    let final_buff = args
        .iter()
        .map(|(v, _)| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!(";; {}", final_buff);
    Ok(Value::Nil)
}

fn print_(args: &[(Value, Span)], _: Span) -> Result<Value, RuntimeError> {
    let final_buff = args
        .iter()
        .map(|(v, _)| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    print!(";; {}", final_buff);
    Ok(Value::Nil)
}

pub fn builtins() -> Vec<(&'static str, Value)> {
    vec![
        ("println", Value::new_builtin("println", print_line)),
        ("print", Value::new_builtin("printl", print_)),
    ]
}
