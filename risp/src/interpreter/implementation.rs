#![allow(dead_code)]

use crate::lexer::{Lexer, Span};
use crate::parser::Parser;
use crate::sema::node::{analyze, AstNode, Node};

use super::env::Env;
use super::value::{Callable, RuntimeError, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new(load_builtins: bool) -> Self {
        // TODO: use load_builtins
        let env = Env::default();
        Self {
            env: Rc::new(RefCell::new(env)),
        }
    }

    pub fn run(&mut self, source: &str) -> Result<Value, RuntimeError> {
        let tokens = Lexer::tokenize(source);
        let cst = Parser::parse(tokens)
            .map_err(|e| RuntimeError::ParseError(format!("{e:?}")))?;
        let nodes = analyze(cst)
            .map_err(|e| RuntimeError::AnalyzeError(format!("{e:?}")))?;
        nodes
            .iter()
            .map(|node| self.eval(node))
            .last()
            .unwrap_or(Ok(Value::Nil))
    }

    fn eval(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Long(n) => Ok(Value::Long(*n)),
            Node::Double(n) => Ok(Value::Double(*n)),
            Node::Bool(b) => Ok(Value::Bool(*b)),
            Node::Nil => Ok(Value::Nil),
            Node::String(s) => Ok(Value::String(s.clone())),
            Node::Keyword(s) => Ok(Value::Keyword(s.clone())),
            Node::Var(id) => self.eval_var(*id, node.span.clone()),
            Node::GlobalVar(name) => self.eval_global_var(name, node.span.clone()),
            Node::If { .. } => self.eval_if(node),
            Node::Let { .. } => self.eval_let(node),
            Node::Fn { .. } => self.eval_fn(node),
            Node::Def { .. } => self.eval_def(node),
            Node::Call { .. } => self.eval_call(node),
            Node::List(elems) => self.eval_list_literal(elems),
            Node::Vector(elems) => self.eval_vector_literal(elems),
            Node::Map(pairs) => self.eval_map_literal(pairs),
        }
    }

    fn eval_var(&self, id: u32, span: Span) -> Result<Value, RuntimeError> {
        match self.env.borrow().get_local(id) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable {
                name: "todo".to_string(),
                span,
            }),
        }
    }

    fn eval_global_var(&self, name: &str, span: Span) -> Result<Value, RuntimeError> {
        match self.env.borrow().get_global(name) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable {
                name: name.to_string(),
                span,
            }),
        }
    }

    fn eval_if(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::If { cond, then, _else } => {
                let cond_val = match self.eval(cond)? {
                    Value::Nil => false,
                    Value::Bool(v) => v,
                    v => {
                        return Err(RuntimeError::TypeError {
                            expected: "bool",
                            got: v.type_name(),
                            span: cond.span.clone(),
                        })
                    }
                };
                if cond_val {
                    self.eval(then)
                } else {
                    match _else {
                        Some(else_node) => self.eval(else_node),
                        None => Ok(Value::Nil),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn eval_let(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Let { bindings, body } => {
                let child = Rc::new(RefCell::new(Env::with_parent(self.env.clone())));
                let saved = std::mem::replace(&mut self.env, child);

                let result = (|| {
                    for (id, val_node) in bindings {
                        let val = self.eval(val_node)?;
                        self.env.borrow_mut().set_local(*id, val);
                    }
                    self.eval(body)
                })();
                self.env = saved;
                result
            }
            _ => unreachable!(),
        }
    }

    fn eval_fn(&self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Fn { params, body } => Ok(Value::Callable(Rc::new(Callable::Closure {
                params: params.clone(),
                body: body.clone(),
                env: self.env.clone(),
            }))),
            _ => unreachable!(),
        }
    }

    fn eval_def(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Def { name, value } => {
                let v = self.eval(value)?;
                self.env.borrow_mut().set_global(name, v);
                Ok(Value::Nil)
            }
            _ => unreachable!(),
        }
    }

    fn eval_call(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Call { callee, args } => {
                let callee_value = self.eval(callee)?;
                match callee_value {
                    Value::Callable(callable) => match callable.as_ref() {
                        Callable::Closure { params, body, env } => {
                            if args.len() != params.len() {
                                return Err(RuntimeError::WrongArity {
                                    expected: params.len(),
                                    got: args.len(),
                                    span: node.span.clone(),
                                });
                            }

                            let child_env = Rc::new(RefCell::new(Env::with_parent(env.clone())));

                            let evaluated_args: Result<Vec<Value>, _> =
                                args.iter().map(|a| self.eval(a)).collect();
                            let evaluated_args = evaluated_args?;

                            for (param_id, value) in params.iter().zip(evaluated_args.into_iter()) {
                                child_env.borrow_mut().set_local(*param_id, value);
                            }

                            let body = body.clone();
                            let saved = std::mem::replace(&mut self.env, child_env);
                            let result = self.eval(&body);
                            self.env = saved;
                            result
                        }
                        Callable::Builtin { name: _, func } => {
                            let evaluated_args: Result<Vec<Value>, _> =
                                args.iter().map(|a| self.eval(a)).collect();
                            let evaluated_args = evaluated_args?;
                            func(&evaluated_args)
                        }
                    },
                    _ => Err(RuntimeError::NotCallable {
                        span: node.span.clone(),
                    }),
                }
            }
            _ => unreachable!(),
        }
    }

    fn eval_list_literal(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        elems
            .iter()
            .map(|e| self.eval(e))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::List)
    }

    fn eval_vector_literal(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut arr_values: Vec<Value> = vec![];
        for elem in elems {
            let value = self.eval(elem)?;
            arr_values.push(value);
        }

        Ok(Value::Vector(arr_values))
    }

    fn eval_map_literal(&mut self, pairs: &[(AstNode, AstNode)]) -> Result<Value, RuntimeError> {
        // Replace Vec<(Value, Value)> with a proper HashMap
        let mut map_values: Vec<(Value, Value)> = vec![];
        for (k, v) in pairs {
            let k_value = self.eval(k)?;
            let v_value = self.eval(v)?;
            map_values.push((k_value, v_value));
        }

        Ok(Value::Map(map_values))
    }
}
