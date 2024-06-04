use std::fmt::Display;

pub enum ASTObject {}
pub struct EvalError {
    err: String,
    ch: usize,
    line: usize,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{line}:{ch} {err}", err = self.err, line = self.line, ch = self.ch)
    }
}
pub trait Eval {
    fn eval(&self) -> Result<(), EvalError>;
}

struct ASTLet {
    name: String,
    
}