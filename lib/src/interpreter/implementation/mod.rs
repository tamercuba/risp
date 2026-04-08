mod eval_call;
mod eval_forms;
mod eval_hof;
mod eval_literals;
mod eval_logic;
mod eval_loop;

use super::builtins::builtins;
pub use crate::interpreter::{Callable, Env, RuntimeError, Value};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::sema::{analyze, AstNode, Node};
use std::cell::RefCell;
use std::rc::Rc;

const SRC_STDLIB_CORE: &str = include_str!("../stdlib/src/core.risp");

pub struct Interpreter {
    pub(super) env: Rc<RefCell<Env>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::new(RefCell::new(Env::default()));
        {
            let e = env.borrow();
            e.load_builtins("risp.internal", builtins());
            e.create_ns("risp.core", vec!["risp.internal"]);
            e.create_ns("user", vec!["risp.core"]);
        }
        let mut interp = Self { env };
        interp
            .run_in_ns(SRC_STDLIB_CORE, "risp.core")
            .expect("core.risp failed to load");
        interp
    }

    fn run_in_ns(&mut self, source: &str, ns: &str) -> Result<Value, RuntimeError> {
        let current_ns = self.env.borrow().get_current_namespace();
        self.env.borrow().set_current_namespace(ns);
        let result = self.run(source);
        self.env.borrow().set_current_namespace(&current_ns);
        result
    }

    pub fn completions(&self) -> Vec<String> {
        let builtins = self.env.borrow().public_names();
        let special_forms = ["if", "let", "fn", "def", "defn", "do", "apply"];
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
            Node::String(s) => Ok(Value::String(Rc::from(s.as_str()))),
            Node::Keyword(s) => Ok(Value::Keyword(Rc::from(s.as_str()))),
            Node::Var(id) => self.eval_var(*id, node.span),
            Node::GlobalVar(name) => self.eval_global_var(name, node.span),
            Node::QualifiedVar { ns, name } => self.eval_qualified_var(ns, name, node.span),
            Node::And(_) | Node::Or(_) => self.eval_logic(node),
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
            Node::Symbol(s) => Ok(Value::Symbol(Rc::from(s.as_str()))),
            Node::Loop { bindings, body } => self.eval_loop(bindings, body),
            Node::Recur(_) => Err(RuntimeError::RecurOutsideLoop { span: node.span }),
        }
    }
}
