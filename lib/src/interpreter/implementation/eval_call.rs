use super::{Callable, Env, Interpreter, RuntimeError, Value};
use crate::interpreter::value::ClosureArity;
use crate::lexer::Span;
use crate::sema::{AstNode, Node};
use std::cell::RefCell;
use std::rc::Rc;

impl Interpreter {
    pub(super) fn call_value(
        &mut self,
        func: &Value,
        args: Vec<(Value, Span)>,
        span: Span,
    ) -> Result<Value, RuntimeError> {
        match func {
            Value::Callable(callable) => match callable.as_ref() {
                Callable::Builtin { func, .. } => func(&args, span),
                Callable::Closure { arities, env } => {
                    let arity = Self::select_arity(arities, args.len(), span)?;
                    let child_env = Rc::new(RefCell::new(Env::with_parent(env.clone())));
                    for (param_id, (value, _)) in arity.params.iter().zip(args.iter()) {
                        child_env.borrow_mut().set_local(*param_id, value.clone());
                    }
                    if let Some(rest_id) = arity.variadic {
                        let rest: Vec<Value> = args[arity.params.len()..]
                            .iter()
                            .map(|(v, _)| v.clone())
                            .collect();
                        let rest_list = rest.into_iter().collect();
                        child_env
                            .borrow_mut()
                            .set_local(rest_id, Value::List(rest_list));
                    }
                    let body = arity.body.clone();
                    let saved = std::mem::replace(&mut self.env, child_env);
                    let result = self.eval(&body);
                    self.env = saved;
                    result
                }
            },
            _ => Err(RuntimeError::NotCallable { span }),
        }
    }

    fn select_arity(
        arities: &[ClosureArity],
        n_args: usize,
        span: Span,
    ) -> Result<&ClosureArity, RuntimeError> {
        arities
            .iter()
            .find(|a| {
                if a.variadic.is_some() {
                    n_args >= a.params.len()
                } else {
                    n_args == a.params.len()
                }
            })
            .ok_or_else(|| RuntimeError::WrongArity {
                expected: arities.iter().map(|a| a.params.len()).min().unwrap_or(0),
                got: n_args,
                span,
            })
    }

    pub(super) fn eval_call(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Call { callee, args } => {
                if let Node::GlobalVar(name) = &callee.node {
                    if name == "apply" {
                        return self.eval_apply(args, node.span);
                    }
                }

                let callee_value = self.eval(callee)?;
                match callee_value {
                    Value::Callable(callable) => {
                        let evaluated_args: Result<Vec<(Value, Span)>, _> =
                            args.iter().map(|a| Ok((self.eval(a)?, a.span))).collect();
                        self.call_value(&Value::Callable(callable), evaluated_args?, node.span)
                    }
                    Value::Keyword(v) => {
                        if args.len() != 1 {
                            return Err(RuntimeError::WrongArity {
                                expected: 1,
                                got: args.len(),
                                span: node.span,
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
                                span: args[0].span,
                            }),
                        }
                    }
                    _ => Err(RuntimeError::NotCallable { span: node.span }),
                }
            }
            _ => unreachable!(),
        }
    }
}
