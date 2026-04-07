use super::{Interpreter, RuntimeError, Value};
use crate::lexer::Span;
use crate::sema::AstNode;

fn to_vec(val: Value, span: Span) -> Result<Vec<Value>, RuntimeError> {
    match val {
        Value::List(l) => Ok(l.iter().cloned().collect()),
        Value::Vector(v) => Ok(v),
        v => Err(RuntimeError::TypeError {
            expected: "list or vector",
            got: v.type_name(),
            span,
        }),
    }
}

impl Interpreter {
    // (apply f arg* coll)
    pub(super) fn eval_apply(
        &mut self,
        args: &[AstNode],
        span: Span,
    ) -> Result<Value, RuntimeError> {
        if args.len() < 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
                span,
            });
        }

        let func = self.eval(&args[0])?;

        let mut all_args: Vec<(Value, Span)> = vec![];
        for arg in &args[1..args.len() - 1] {
            all_args.push((self.eval(arg)?, arg.span));
        }

        let last = &args[args.len() - 1];
        let last_items = to_vec(self.eval(last)?, last.span)?;
        all_args.extend(last_items.into_iter().map(|v| (v, last.span)));

        self.call_value(&func, all_args, span)
    }

}
