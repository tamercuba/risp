use super::{Callable, Env, Interpreter, RuntimeError, Value};
use crate::lexer::Span;
use crate::sema::{AstNode, Node};
use std::cell::RefCell;
use std::rc::Rc;

impl Interpreter {
    pub(super) fn call_value(
        &mut self,
        func: Value,
        args: Vec<(Value, Span)>,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        match func {
            Value::Callable(callable) => match callable.as_ref() {
                Callable::Builtin { func, .. } => func(&args, span),
                Callable::Closure { params, body, env } => {
                    if args.len() != params.len() {
                        return Err(RuntimeError::WrongArity {
                            expected: params.len(),
                            got: args.len(),
                            span,
                        });
                    }
                    let child_env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
                    for (param_id, (value, _)) in params.iter().zip(args) {
                        child_env.borrow_mut().set_local(*param_id, value);
                    }
                    let body = body.clone();
                    let saved = std::mem::replace(&mut self.env, child_env);
                    let result = self.eval(&body);
                    self.env = saved;
                    result
                }
            },
            _ => Err(RuntimeError::NotCallable { span }),
        }
    }

    pub(super) fn eval_call(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Call { callee, args } => {
                if let Node::GlobalVar(name) = &callee.node {
                    match name.as_str() {
                        "apply" => return self.eval_apply(args, node.span.clone()),
                        "map" => return self.eval_map_hof(args, node.span.clone()),
                        "filter" => return self.eval_filter(args, node.span.clone()),
                        "reduce" => return self.eval_reduce(args, node.span.clone()),
                        _ => {}
                    }
                }

                let callee_value = self.eval(callee)?;
                match callee_value {
                    Value::Callable(callable) => {
                        let evaluated_args: Result<Vec<(Value, Span)>, _> = args
                            .iter()
                            .map(|a| Ok((self.eval(a)?, a.span.clone())))
                            .collect();
                        self.call_value(
                            Value::Callable(callable),
                            evaluated_args?,
                            node.span.clone(),
                        )
                    }
                    Value::Keyword(v) => {
                        if args.len() != 1 {
                            return Err(RuntimeError::WrongArity {
                                expected: 1,
                                got: args.len(),
                                span: node.span.clone(),
                            });
                        }
                        let arg = self.eval(&args[0])?;
                        match arg {
                            Value::Map(pairs) => Ok(pairs
                                .into_iter()
                                .find(|(key, _)| matches!(key, Value::Keyword(s) if *s == v))
                                .map(|(_, v)| v)
                                .unwrap_or(Value::Nil)),
                            _ => Err(RuntimeError::TypeError {
                                expected: "map",
                                got: arg.type_name(),
                                span: args[0].span.clone(),
                            }),
                        }
                    }
                    _ => Err(RuntimeError::NotCallable {
                        span: node.span.clone(),
                    }),
                }
            }
            _ => unreachable!(),
        }
    }
}
