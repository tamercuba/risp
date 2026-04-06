use super::{Interpreter, RuntimeError, Value};
use crate::collections::RispList;
use crate::lexer::Span;
use crate::sema::AstNode;

fn seq_to_list(v: Value) -> Value {
    match v {
        Value::Vector(vec) => Value::List(vec.into_iter().collect()),
        other => other,
    }
}

impl Interpreter {
    pub(super) fn eval_var(&self, id: u32, span: Span) -> Result<Value, RuntimeError> {
        match self.env.borrow().get_local(id) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable {
                name: "todo".to_string(),
                span,
            }),
        }
    }

    pub(super) fn eval_global_var(&self, name: &str, span: Span) -> Result<Value, RuntimeError> {
        match self.env.borrow().get_global(name) {
            Some(v) => Ok(v),
            None => Err(RuntimeError::UndefinedVariable {
                name: name.to_string(),
                span,
            }),
        }
    }

    pub(super) fn eval_list_literal(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        let values = elems
            .iter()
            .map(|e| self.eval(e))
            .collect::<Result<Vec<_>, _>>()?;

        let list = values
            .into_iter()
            .rev()
            .fold(RispList::empty(), |acc, val| RispList::cons(val, &acc));

        Ok(Value::List(list))
    }

    pub(super) fn eval_vector_literal(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        elems
            .iter()
            .map(|e| self.eval(e))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Vector)
    }

    pub(super) fn eval_map_literal(
        &mut self,
        pairs: &[(AstNode, AstNode)],
    ) -> Result<Value, RuntimeError> {
        pairs
            .iter()
            .map(|(k, v)| Ok((self.eval(k)?, self.eval(v)?)))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Map)
    }

    pub(super) fn eval_set_literal(&mut self, elems: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut values: Vec<Value> = vec![];
        for elem in elems {
            let v = seq_to_list(self.eval(elem)?);
            if !values.contains(&v) {
                values.push(v);
            }
        }
        Ok(Value::Set(values))
    }
}
