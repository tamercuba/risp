use std::cell::RefCell;
use std::rc::Rc;

use super::{Callable, Env, Interpreter, RuntimeError, Value};
use crate::sema::{AstNode, LocalId, Node};

impl Interpreter {
    pub(super) fn eval_bindings_with_toplevel_frame(
        &mut self,
        bindings: &[(LocalId, AstNode)],
    ) -> Result<Option<Rc<RefCell<Env>>>, RuntimeError> {
        let max_id = bindings
            .iter()
            .map(|(id, _)| *id as usize)
            .max()
            .unwrap_or(0);
        if self.env.borrow().frame_len() > max_id {
            // Inside a function frame: slots already allocated, write directly
            for (id, val_node) in bindings.iter() {
                let val = self.eval(val_node)?;
                self.env.borrow_mut().set_local(*id, val);
            }
            Ok(None)
        } else {
            // Top-level: no function frame exists, allocate a temporary one
            let child_env = Rc::new(RefCell::new(Env::with_frame(self.env.clone(), max_id + 1)));
            let saved = std::mem::replace(&mut self.env, child_env);
            for (id, val_node) in bindings.iter() {
                let val = self.eval(val_node)?;
                self.env.borrow_mut().set_local(*id, val);
            }
            Ok(Some(saved))
        }
    }

    pub(super) fn eval_if(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::If { cond, then, _else } => {
                let cond_val = match self.eval(cond)? {
                    Value::Nil => false,
                    Value::Bool(v) => v,
                    v => {
                        return Err(RuntimeError::TypeError {
                            expected: "bool",
                            got: v.type_name(),
                            span: cond.span,
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

    pub(super) fn eval_let(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Let { bindings, body } => {
                let saved = self.eval_bindings_with_toplevel_frame(bindings)?;
                let result = self.eval(body);
                if let Some(env) = saved {
                    self.env = env;
                }
                result
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn eval_fn(&self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Fn { arities } => Ok(Value::Callable(Rc::new(Callable::Closure {
                arities: arities.iter().map(Into::into).collect(),
                env: self.env.clone(),
                name: None,
            }))),
            _ => unreachable!(),
        }
    }

    pub(super) fn eval_def(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Def { name, value } => {
                let v = self.eval(value)?;
                let v = match v {
                    Value::Callable(ref c) => match c.as_ref() {
                        Callable::Closure {
                            arities,
                            env,
                            name: None,
                        } => Value::Callable(Rc::new(Callable::Closure {
                            arities: arities.clone(),
                            env: env.clone(),
                            name: Some(name.clone()),
                        })),
                        _ => v,
                    },
                    _ => v,
                };
                self.env.borrow_mut().set_global(name, v);
                Ok(Value::Nil)
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn eval_do(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        elems
            .iter()
            .map(|e| self.eval(e))
            .last()
            .unwrap_or(Ok(Value::Nil))
    }
}
