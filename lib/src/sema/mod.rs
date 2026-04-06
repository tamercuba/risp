mod ast_scope;
mod node;

use std::rc::Rc;

use self::ast_scope::Scope;
pub use self::node::{AnalyzeError, AstNode, Node};
use crate::lexer::Span;
use crate::parser::{Expr, ExprKind};

#[cfg(test)]
mod test_sema;

pub fn analyze(cst: Vec<Expr>) -> Result<Vec<AstNode>, AnalyzeError> {
    let scope = Scope::new();
    cst.into_iter()
        .map(|expr| analyze_expr(expr, &scope))
        .collect()
}

fn analyze_expr(expr: Expr, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    let span = expr.span;
    match expr.kind {
        ExprKind::Long(n) => Ok(AstNode::new(Node::Long(n), span)),
        ExprKind::Double(n) => Ok(AstNode::new(Node::Double(n), span)),
        ExprKind::Bool(b) => Ok(AstNode::new(Node::Bool(b), span)),
        ExprKind::Nil => Ok(AstNode::new(Node::Nil, span)),
        ExprKind::String(s) => Ok(AstNode::new(Node::String(s), span)),
        ExprKind::Keyword(s) => Ok(AstNode::new(Node::Keyword(s), span)),
        ExprKind::Symbol(s) => match scope.get_by_name(&s) {
            Some(id) => Ok(AstNode::new(Node::Var(id), span)),
            None => Ok(AstNode::new(Node::GlobalVar(s), span)),
        },
        ExprKind::List(elems) => analyze_list(elems, span, scope),
        ExprKind::Vector(elems) => {
            let nodes = elems
                .into_iter()
                .map(|expr| analyze_expr(expr, scope))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(Node::Vector(nodes), span))
        }
        ExprKind::Map(pairs) => {
            let nodes = pairs
                .into_iter()
                .map(|(k, v)| Ok((analyze_expr(k, scope)?, analyze_expr(v, scope)?)))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(Node::Map(nodes), span))
        }
        ExprKind::Set(elems) => {
            let nodes = elems
                .into_iter()
                .map(|e| analyze_expr(e, scope))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(Node::Set(nodes), span))
        }
        ExprKind::Quote(inner) => analyze_quoted(*inner, scope),
    }
}

fn analyze_quoted(expr: Expr, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    let span = expr.span;
    match expr.kind {
        ExprKind::Symbol(s) => Ok(AstNode::new(Node::Symbol(s), span)),
        ExprKind::List(elems) => {
            let nodes = elems
                .into_iter()
                .map(|e| analyze_quoted(e, scope))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(Node::List(nodes), span))
        }
        ExprKind::Vector(elems) => {
            let nodes = elems
                .into_iter()
                .map(|e| analyze_quoted(e, scope))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::new(Node::Vector(nodes), span))
        }
        // Literals pass through normally
        _ => analyze_expr(expr, scope),
    }
}

fn is_symbol(expr: &Expr, name: &str) -> bool {
    matches!(&expr.kind, ExprKind::Symbol(s) if s == name)
}

fn analyze_list(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    match elems.first() {
        Some(head) if is_symbol(head, "if") => analyze_if(elems, span, scope),
        Some(head) if is_symbol(head, "let") => analyze_let(elems, span, scope),
        Some(head) if is_symbol(head, "fn") => analyze_fn(elems, span, scope),
        Some(head) if is_symbol(head, "defn") => analyze_defn(elems, span, scope),
        Some(head) if is_symbol(head, "def") => analyze_def(elems, span, scope),
        Some(head) if is_symbol(head, "do") => analyze_do(elems, span, scope),
        _ => analyze_call(elems, span, scope),
    }
}

fn analyze_do(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (do () () ()) return last Expr
    let body: Vec<AstNode> = elems[1..]
        .iter()
        .map(|e| analyze_expr(e.clone(), scope))
        .collect::<Result<_, _>>()?;
    Ok(AstNode::new(Node::Do(body), span))
}

fn analyze_def(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (def x bla)
    if elems.len() != 3 {
        return Err(AnalyzeError::InvalidArity { form: "def", span });
    }

    let symbol_expr = elems[1].clone();
    let name = match (symbol_expr.kind, symbol_expr.span) {
        (ExprKind::Symbol(name), _) => Ok(name),
        (_, s) => Err(AnalyzeError::InvalidBindingKey(s)),
    }?;

    let value = Box::new(analyze_expr(elems[2].clone(), scope)?);

    Ok(AstNode::new(Node::Def { name, value }, span))
}

fn analyze_if(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    if elems.len() < 3 || elems.len() > 4 {
        return Err(AnalyzeError::InvalidArity { form: "if", span });
    }

    // 0 is the if symbol
    let cond = analyze_expr(elems[1].clone(), scope)?;
    let then = analyze_expr(elems[2].clone(), scope)?;
    let _else = elems
        .into_iter()
        .nth(3)
        .map(|e| analyze_expr(e, scope).map(Box::new))
        .transpose()?;

    Ok(AstNode::new(
        Node::If {
            cond: Box::new(cond),
            then: Box::new(then),
            _else,
        },
        span,
    ))
}

fn analyze_let(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (let [a 1] body)
    if elems.len() != 3 {
        return Err(AnalyzeError::InvalidArity { form: "let", span });
    }

    // 0 is the let symbol
    let bindings_expr = elems[1].clone();
    let bindings_span = bindings_expr.span;

    let mut child_scope = scope.enter_scope();
    let bindings_array: Vec<Expr> = match bindings_expr.kind {
        ExprKind::Vector(l) => Ok(l),
        _ => Err(AnalyzeError::InvalidBindings(bindings_span)),
    }?;

    if !bindings_array.len().is_multiple_of(2) {
        return Err(AnalyzeError::OddBindings(bindings_span));
    }

    let mut iter = bindings_array.into_iter();
    let mut bindings: Vec<(u32, AstNode)> = vec![];

    while let (Some(k), Some(v)) = (iter.next(), iter.next()) {
        let key = match k.kind {
            ExprKind::Symbol(name) => Ok(child_scope.bind(name)),
            _ => Err(AnalyzeError::InvalidBindingKey(k.span)),
        }?;
        let val = analyze_expr(v, &child_scope)?;
        bindings.push((key, val));
    }

    let body = Box::new(analyze_expr(elems[2].clone(), &child_scope)?);

    Ok(AstNode::new(Node::Let { bindings, body }, span))
}

fn analyze_fn(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (fn [a b] (...))
    if elems.len() != 3 {
        return Err(AnalyzeError::InvalidArity { form: "fn", span });
    }

    let mut child_scope = scope.enter_scope();
    let params_expr = elems[1].clone();
    let params_values: Vec<Expr> = match (params_expr.kind, params_expr.span) {
        (ExprKind::Vector(v), _) => Ok(v),
        (_, params_span) => Err(AnalyzeError::InvalidFnParams(params_span)),
    }?;
    let params: Vec<u32> = params_values
        .into_iter()
        .map(|e| match e.kind {
            ExprKind::Symbol(name) => Ok(child_scope.bind(name)),
            _ => Err(AnalyzeError::InvalidFnParams(e.span)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let body = Rc::new(analyze_expr(elems[2].clone(), &child_scope)?);

    Ok(AstNode::new(Node::Fn { params, body }, span))
}

fn analyze_defn(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (defn bla [a b] (...))

    if elems.len() != 4 {
        return Err(AnalyzeError::InvalidArity { form: "defn", span });
    }

    let symbol_expr = elems[1].clone();
    let name = match (symbol_expr.kind, symbol_expr.span) {
        (ExprKind::Symbol(name), _) => Ok(name),
        (_, s) => Err(AnalyzeError::InvalidBindingKey(s)),
    }?;

    let params_expr = elems[2].clone();
    let params_values: Vec<Expr> = match (params_expr.kind, params_expr.span) {
        (ExprKind::Vector(v), _) => Ok(v),
        (_, params_span) => Err(AnalyzeError::InvalidFnParams(params_span)),
    }?;
    let mut child_scope = scope.enter_scope();
    let params: Vec<u32> = params_values
        .into_iter()
        .map(|e| match e.kind {
            ExprKind::Symbol(name) => Ok(child_scope.bind(name)),
            _ => Err(AnalyzeError::InvalidFnParams(e.span)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let body = Rc::new(analyze_expr(elems[3].clone(), &child_scope)?);
    let value = Box::new(AstNode::new(Node::Fn { params, body }, span));

    Ok(AstNode::new(Node::Def { name, value }, span))
}

fn analyze_call(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    if elems.is_empty() {
        return Err(AnalyzeError::InvalidExpression(span));
    }

    let callee = Box::new(analyze_expr(elems[0].clone(), scope)?);
    let args = elems[1..]
        .iter()
        .map(|e| analyze_expr(e.clone(), scope))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(AstNode::new(Node::Call { callee, args }, span))
}
