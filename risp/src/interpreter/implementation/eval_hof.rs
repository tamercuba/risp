use super::{Interpreter, RuntimeError, Value};
use crate::lexer::Span;
use crate::sema::AstNode;

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
            all_args.push((self.eval(arg)?, arg.span.clone()));
        }

        let last = &args[args.len() - 1];
        match self.eval(last)? {
            Value::List(v) | Value::Vector(v) => {
                all_args.extend(v.into_iter().map(|v| (v, last.span.clone())));
            }
            v => {
                return Err(RuntimeError::TypeError {
                    expected: "list or vector",
                    got: v.type_name(),
                    span: last.span.clone(),
                })
            }
        }

        self.call_value(func, all_args, span)
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
        let coll_span = args[1].span.clone();

        let items = match self.eval(&args[1])? {
            Value::List(v) | Value::Vector(v) => Ok(v),
            v => Err(RuntimeError::TypeError {
                expected: "list or vector",
                got: v.type_name(),
                span: coll_span.clone(),
            }),
        }?;

        let result = items
            .into_iter()
            .map(|v| self.call_value(func.clone(), vec![(v, coll_span.clone())], span.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Value::List(result))
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
        let coll_span = args[1].span.clone();

        let items = match self.eval(&args[1])? {
            Value::List(v) | Value::Vector(v) => Ok(v),
            v => Err(RuntimeError::TypeError {
                expected: "list or vector",
                got: v.type_name(),
                span: coll_span.clone(),
            }),
        }?;

        let mut result = vec![];
        for v in items {
            let keep = !matches!(
                self.call_value(
                    func.clone(),
                    vec![(v.clone(), coll_span.clone())],
                    span.clone()
                )?,
                Value::Nil | Value::Bool(false)
            );
            if keep {
                result.push(v);
            }
        }

        Ok(Value::List(result))
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
            let coll_span = args[2].span.clone();
            let items = match self.eval(&args[2])? {
                Value::List(v) | Value::Vector(v) => Ok(v),
                v => Err(RuntimeError::TypeError {
                    expected: "list or vector",
                    got: v.type_name(),
                    span: coll_span,
                }),
            }?;
            (init, items)
        } else {
            let coll_span = args[1].span.clone();
            let mut items = match self.eval(&args[1])? {
                Value::List(v) | Value::Vector(v) => Ok(v),
                v => Err(RuntimeError::TypeError {
                    expected: "list or vector",
                    got: v.type_name(),
                    span: coll_span,
                }),
            }?;
            if items.is_empty() {
                return Ok(Value::Nil);
            }
            let first = items.remove(0);
            (first, items)
        };

        for v in items {
            let elem_span = args[args.len() - 1].span.clone();
            acc = self.call_value(
                func.clone(),
                vec![(acc, span.clone()), (v, elem_span)],
                span.clone(),
            )?;
        }

        Ok(acc)
    }
}
