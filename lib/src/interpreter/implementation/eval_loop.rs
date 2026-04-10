use std::cell::RefCell;
use std::rc::Rc;

use super::{Env, Interpreter, RuntimeError, Value};
use crate::interpreter::value::EvalFlow;
use crate::sema::{AstNode, LocalId, Node};

impl Interpreter {
    fn eval_flow(&mut self, node: &AstNode) -> Result<EvalFlow, RuntimeError> {
        match &node.node {
            Node::Recur(args) => {
                let vals = args
                    .iter()
                    .map(|a| self.eval(a))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(EvalFlow::Recur(vals))
            }
            Node::If { cond, then, _else } => {
                let cond_val = self.eval(cond)?;
                let is_truthy = !matches!(cond_val, Value::Bool(false) | Value::Nil);
                if is_truthy {
                    self.eval_flow(then)
                } else if let Some(else_branch) = _else {
                    self.eval_flow(else_branch)
                } else {
                    Ok(EvalFlow::Value(Value::Nil))
                }
            }
            Node::Do(elems) => {
                let (last, rest) = match elems.split_last() {
                    Some(parts) => parts,
                    None => return Ok(EvalFlow::Value(Value::Nil)),
                };
                for elem in rest {
                    self.eval(elem)?;
                }
                self.eval_flow(last)
            }
            Node::Let { bindings, body } => {
                let child_env = Rc::new(RefCell::new(Env::with_parent(self.env.clone())));
                for (id, v_node) in bindings.iter() {
                    let value = self.eval(v_node)?;
                    child_env.borrow_mut().set_local(*id, value);
                }
                let saved = std::mem::replace(&mut self.env, child_env);
                let result = self.eval_flow(body);
                self.env = saved;
                result
            }
            _ => self.eval(node).map(EvalFlow::Value),
        }
    }
    pub(super) fn eval_loop(
        &mut self,
        bindings: &[(LocalId, AstNode)],
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let child_env = Rc::new(RefCell::new(Env::with_parent(self.env.clone())));
        for (id, v_node) in bindings.iter() {
            let value = self.eval(v_node)?;
            child_env.borrow_mut().set_local(*id, value);
        }
        let saved = std::mem::replace(&mut self.env, child_env);

        let result = loop {
            match self.eval_flow(body) {
                Err(e) => break Err(e),
                Ok(EvalFlow::Value(v)) => break Ok(v),
                Ok(EvalFlow::Recur(new_vals)) => {
                    for ((id, _), new_val) in bindings.iter().zip(new_vals) {
                        self.env.borrow_mut().set_local(*id, new_val);
                    }
                }
            }
        };

        self.env = saved;
        result
    }
}
