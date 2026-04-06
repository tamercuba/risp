use super::{Interpreter, RuntimeError, Value};
use crate::collections::RispList;
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

    // (map f coll)
    pub(super) fn eval_map_hof(
        &mut self,
        args: &[AstNode],
        span: Span,
    ) -> Result<Value, RuntimeError> {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
                span,
            });
        }

        let func = self.eval(&args[0])?;
        let coll_span = args[1].span;
        let items = to_vec(self.eval(&args[1])?, coll_span)?;

        let result = items
            .into_iter()
            .map(|v| self.call_value(&func, vec![(v, coll_span)], span))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Value::Vector(result))
    }

    // (filter f coll)
    pub(super) fn eval_filter(
        &mut self,
        args: &[AstNode],
        span: Span,
    ) -> Result<Value, RuntimeError> {
        if args.len() != 2 {
            return Err(RuntimeError::WrongArity {
                expected: 2,
                got: args.len(),
                span,
            });
        }

        let func = self.eval(&args[0])?;
        let coll_span = args[1].span;
        let items = to_vec(self.eval(&args[1])?, coll_span)?;

        let mut result = vec![];
        for v in items {
            let keep = !matches!(
                self.call_value(&func, vec![(v.clone(), coll_span)], span)?,
                Value::Nil | Value::Bool(false)
            );
            if keep {
                result.push(v);
            }
        }

        Ok(Value::List(
            result
                .into_iter()
                .rev()
                .fold(RispList::empty(), |acc, v| RispList::cons(v, &acc)),
        ))
    }

    // (reduce f init coll) ou (reduce f coll)
    pub(super) fn eval_reduce(
        &mut self,
        args: &[AstNode],
        span: Span,
    ) -> Result<Value, RuntimeError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(RuntimeError::WrongArity {
                expected: 3,
                got: args.len(),
                span,
            });
        }

        let func = self.eval(&args[0])?;

        let (mut acc, items) = if args.len() == 3 {
            let init = self.eval(&args[1])?;
            let items = to_vec(self.eval(&args[2])?, args[2].span)?;
            (init, items)
        } else {
            let mut items = to_vec(self.eval(&args[1])?, args[1].span)?;
            if items.is_empty() {
                return Ok(Value::Nil);
            }
            let first = items.remove(0);
            (first, items)
        };

        for v in items {
            let elem_span = args[args.len() - 1].span;
            acc = self.call_value(&func, vec![(acc, span), (v, elem_span)], span)?;
        }

        Ok(acc)
    }
}
