mod eval_call;
mod eval_forms;
mod eval_hof;
mod eval_literals;
mod eval_loop;

use super::builtins::builtins;
pub use crate::interpreter::{Callable, Env, RuntimeError, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::sema::{analyze, AstNode, Node};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    pub(super) env: Rc<RefCell<Env>>,
}

impl Interpreter {
    pub fn new(load_builtins: bool) -> Self {
        let env = Rc::new(RefCell::new(Env::default()));
        if load_builtins {
            Self::load_builtins(env.clone());
        }
        Self { env }
    }

    fn load_builtins(env: Rc<RefCell<Env>>) -> Option<()> {
        for (name, value) in builtins() {
            env.borrow_mut().set_global(name, value);
        }
        None
    }

    pub fn completions(&self) -> Vec<String> {
        let builtins = self.env.borrow().global_names();
        let special_forms = [
            "if", "let", "fn", "def", "defn", "do", "apply", "map", "filter", "reduce",
        ];
        special_forms
            .iter()
            .map(|s| s.to_string())
            .chain(builtins)
            .collect()
    }

    pub fn run(&mut self, source: &str) -> Result<Value, RuntimeError> {
        let tokens = Lexer::tokenize(source);
        let cst = Parser::parse(tokens).map_err(|e| RuntimeError::ParseError(format!("{e}")))?;
        let nodes = analyze(cst).map_err(|e| RuntimeError::AnalyzeError(format!("{e}")))?;
        nodes
            .iter()
            .map(|node| self.eval(node))
            .last()
            .unwrap_or(Ok(Value::Nil))
    }

    pub(super) fn eval(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match &node.node {
            Node::Long(n) => Ok(Value::Long(*n)),
            Node::Double(n) => Ok(Value::Double(*n)),
            Node::Bool(b) => Ok(Value::Bool(*b)),
            Node::Nil => Ok(Value::Nil),
            Node::String(s) => Ok(Value::String(s.clone())),
            Node::Keyword(s) => Ok(Value::Keyword(s.clone())),
            Node::Var(id) => self.eval_var(*id, node.span),
            Node::GlobalVar(name) => self.eval_global_var(name, node.span),
            Node::If { .. } => self.eval_if(node),
            Node::Let { .. } => self.eval_let(node),
            Node::Fn { .. } => self.eval_fn(node),
            Node::Def { .. } => self.eval_def(node),
            Node::Call { .. } => self.eval_call(node),
            Node::Do(elems) => self.eval_do(elems),
            Node::List(elems) => self.eval_list_literal(elems),
            Node::Vector(elems) => self.eval_vector_literal(elems),
            Node::Map(pairs) => self.eval_map_literal(pairs),
            Node::Set(elems) => self.eval_set_literal(elems),
            Node::Symbol(s) => Ok(Value::Symbol(s.clone())),
            Node::Loop { bindings, body } => self.eval_loop(bindings, body),
            Node::Recur(_) => Err(RuntimeError::RecurOutsideLoop { span: node.span }),
        }
    }
}
