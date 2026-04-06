use std::rc::Rc;

use crate::lexer::Span;

#[derive(Debug)]
pub enum AnalyzeError {
    InvalidArity { form: &'static str, span: Span },
    InvalidFnParams(Span),
    InvalidBindings(Span),
    OddBindings(Span),
    InvalidBindingKey(Span),
    InvalidExpression(Span),
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzeError::InvalidArity { form, span } => {
                write!(f, "(invalid-arity :form '{form}' :at {})", span.lo)
            }
            AnalyzeError::InvalidFnParams(span) => {
                write!(f, "(invalid-fn-params :at {})", span.lo)
            }
            AnalyzeError::InvalidBindings(span) => {
                write!(f, "(invalid-bindings :at {})", span.lo)
            }
            AnalyzeError::OddBindings(span) => {
                write!(f, "(odd-bindings :at {})", span.lo)
            }
            AnalyzeError::InvalidBindingKey(span) => {
                write!(f, "(invalid-binding-key :at {})", span.lo)
            }
            AnalyzeError::InvalidExpression(span) => {
                write!(f, "(invalid-expression :at {})", span.lo)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AstNode {
    pub node: Node,
    pub span: Span,
}

impl AstNode {
    pub fn new(node: Node, span: Span) -> Self {
        AstNode { node, span }
    }
}

#[derive(Debug, Clone)]
pub struct FnArity {
    pub params: Vec<u32>,
    pub variadic: Option<u32>,
    pub body: Rc<AstNode>,
}

#[derive(Debug, Clone)]
pub enum Node {
    Long(i64),
    Double(f64),
    Bool(bool),
    Nil,
    String(String),
    Keyword(String),

    Var(u32),
    GlobalVar(String),

    Def {
        name: String,
        value: Box<AstNode>,
    },
    Let {
        bindings: Vec<(u32, AstNode)>,
        body: Box<AstNode>,
    },
    Fn {
        arities: Vec<FnArity>,
    },
    Call {
        callee: Box<AstNode>,
        args: Vec<AstNode>,
    },
    If {
        cond: Box<AstNode>,
        then: Box<AstNode>,
        _else: Option<Box<AstNode>>,
    },
    Do(Vec<AstNode>),

    Loop {
        bindings: Vec<(u32, AstNode)>,
        body: Box<AstNode>,
    },
    Recur(Vec<AstNode>),

    List(Vec<AstNode>),
    Vector(Vec<AstNode>),
    Map(Vec<(AstNode, AstNode)>),
    Set(Vec<AstNode>),
    Symbol(String),
}
