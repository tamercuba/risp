use crate::interpreter::value::{RuntimeError, Value};
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

pub fn stdio_builtins() -> Vec<(&'static str, Value)> {
    vec![("println", Value::new_builtin("println", print_line))]
}
