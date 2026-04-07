use super::{Interpreter, RuntimeError, Value};
use crate::sema::{AstNode, Node};

impl Interpreter {
    fn eval_and(&mut self, args: &[AstNode]) -> Result<Value, RuntimeError> {
        if args.is_empty() {
            return Ok(Value::Bool(true));
        }
        for arg in &args[..args.len() - 1] {
            let v = self.eval(arg)?;
            if !v.is_truthy() {
                return Ok(v);
            }
        }
        self.eval(args.last().unwrap())
    }

    fn eval_or(&mut self, args: &[AstNode]) -> Result<Value, RuntimeError> {
        for arg in args {
            let v = self.eval(arg)?;
            if v.is_truthy() {
                return Ok(v);
            }
        }
        Ok(Value::Nil)
    }

    pub(super) fn eval_logic(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::And(args) => self.eval_and(args),
            Node::Or(args) => self.eval_or(args),
            _ => unreachable!(),
        }
    }
}
