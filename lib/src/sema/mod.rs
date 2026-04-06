mod ast_scope;
mod node;

use std::rc::Rc;

use self::ast_scope::Scope;
pub use self::node::{AnalyzeError, AstNode, FnArity, Node};
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
        Some(head) if is_symbol(head, "loop") => analyze_loop(elems, span, scope),
        Some(head) if is_symbol(head, "recur") => analyze_recur(elems, span, scope),
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

fn analyze_fn_params(
    params_expr: Expr,
    scope: &mut Scope,
) -> Result<(Vec<u32>, Option<u32>), AnalyzeError> {
    let params_span = params_expr.span;
    let param_exprs: Vec<Expr> = match params_expr.kind {
        ExprKind::Vector(v) => Ok(v),
        _ => Err(AnalyzeError::InvalidFnParams(params_span)),
    }?;

    let amp_pos = param_exprs
        .iter()
        .position(|e| matches!(&e.kind, ExprKind::Symbol(s) if s == "&"));

    let (fixed_exprs, variadic_name) = match amp_pos {
        None => (param_exprs, None),
        Some(pos) => {
            let rest = &param_exprs[pos + 1..];
            if rest.len() != 1 {
                return Err(AnalyzeError::InvalidFnParams(params_span));
            }
            let name = match rest[0].kind.clone() {
                ExprKind::Symbol(name) => Ok(name),
                _ => Err(AnalyzeError::InvalidFnParams(rest[0].span)),
            }?;
            (param_exprs[..pos].to_vec(), Some(name))
        }
    };

    let params = fixed_exprs
        .into_iter()
        .map(|e| match e.kind {
            ExprKind::Symbol(name) => Ok(scope.bind(name)),
            _ => Err(AnalyzeError::InvalidFnParams(e.span)),
        })
        .collect::<Result<_, _>>()?;

    let variadic = variadic_name.map(|name| scope.bind(name));

    Ok((params, variadic))
}

fn analyze_fn_arity(
    params_expr: Expr,
    body_expr: Expr,
    scope: &Scope,
) -> Result<FnArity, AnalyzeError> {
    let mut child_scope = scope.enter_scope();
    let (params, variadic) = analyze_fn_params(params_expr, &mut child_scope)?;
    let body = Rc::new(analyze_expr(body_expr, &child_scope)?);
    Ok(FnArity {
        params,
        variadic,
        body,
    })
}

fn validate_arities(arities: &[FnArity], span: Span) -> Result<(), AnalyzeError> {
    let mut seen_fixed_counts: Vec<usize> = vec![];
    let mut variadic_count = 0usize;

    for arity in arities {
        if arity.variadic.is_some() {
            variadic_count += 1;
        } else {
            let n = arity.params.len();
            if seen_fixed_counts.contains(&n) {
                return Err(AnalyzeError::InvalidFnParams(span));
            }
            seen_fixed_counts.push(n);
        }
    }

    if variadic_count > 1 {
        return Err(AnalyzeError::InvalidFnParams(span));
    }

    Ok(())
}

fn analyze_arities(
    arity_exprs: &[Expr],
    form: &'static str,
    span: Span,
    scope: &Scope,
) -> Result<Vec<FnArity>, AnalyzeError> {
    match arity_exprs {
        [] => Err(AnalyzeError::InvalidArity { form, span }),
        // single-arity sugar sem body: (fn [params])
        [only] if matches!(only.kind, ExprKind::Vector(_)) => {
            Err(AnalyzeError::InvalidArity { form, span })
        }
        // single-arity sugar: (fn [params] body)
        [params, body] if matches!(params.kind, ExprKind::Vector(_)) => {
            Ok(vec![analyze_fn_arity(params.clone(), body.clone(), scope)?])
        }
        // multi-arity: (fn ([params] body) ...)
        exprs => exprs
            .iter()
            .map(|arity_expr| match &arity_expr.kind {
                ExprKind::List(elems) if elems.len() == 2 => {
                    analyze_fn_arity(elems[0].clone(), elems[1].clone(), scope)
                }
                _ => Err(AnalyzeError::InvalidFnParams(arity_expr.span)),
            })
            .collect(),
    }
}

fn analyze_fn(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    if elems.len() < 2 {
        return Err(AnalyzeError::InvalidArity { form: "fn", span });
    }
    let arities = analyze_arities(&elems[1..], "fn", span, scope)?;
    validate_arities(&arities, span)?;
    Ok(AstNode::new(Node::Fn { arities }, span))
}

fn analyze_defn(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (defn name [params] body)          — single arity
    // (defn name ([params] body) ...)    — multi-arity
    if elems.len() < 3 {
        return Err(AnalyzeError::InvalidArity { form: "defn", span });
    }

    let name = match (elems[1].kind.clone(), elems[1].span) {
        (ExprKind::Symbol(name), _) => Ok(name),
        (_, s) => Err(AnalyzeError::InvalidBindingKey(s)),
    }?;

    let arities = analyze_arities(&elems[2..], "defn", span, scope)?;
    validate_arities(&arities, span)?;

    let value = Box::new(AstNode::new(Node::Fn { arities }, span));
    Ok(AstNode::new(Node::Def { name, value }, span))
}

fn analyze_loop(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (loop [x 0] body)
    if elems.len() != 3 {
        return Err(AnalyzeError::InvalidArity { form: "loop", span });
    }

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
    Ok(AstNode::new(Node::Loop { bindings, body }, span))
}

fn analyze_recur(elems: Vec<Expr>, span: Span, scope: &Scope) -> Result<AstNode, AnalyzeError> {
    // (recur expr...)
    let args = elems[1..]
        .iter()
        .map(|e| analyze_expr(e.clone(), scope))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AstNode::new(Node::Recur(args), span))
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
