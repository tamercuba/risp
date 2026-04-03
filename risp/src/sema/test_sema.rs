#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::sema::node::{analyze, AnalyzeError, AstNode, Node};

    fn parse(input: &str) -> Vec<AstNode> {
        let cst = Parser::parse(Lexer::tokenize(input)).unwrap();
        analyze(cst).unwrap()
    }

    fn parse_err(input: &str) -> AnalyzeError {
        let cst = Parser::parse(Lexer::tokenize(input)).unwrap();
        analyze(cst).unwrap_err()
    }

    #[test]
    fn analyzes_long() {
        let result = parse("42");
        assert!(matches!(result[0].node, Node::Long(42)));
    }

    #[test]
    fn analyzes_double() {
        let result = parse("3.14");
        assert!(matches!(result[0].node, Node::Double(_)));
    }

    #[test]
    fn analyzes_bool_true() {
        let result = parse("true");
        assert!(matches!(result[0].node, Node::Bool(true)));
    }

    #[test]
    fn analyzes_bool_false() {
        let result = parse("false");
        assert!(matches!(result[0].node, Node::Bool(false)));
    }

    #[test]
    fn analyzes_nil() {
        let result = parse("nil");
        assert!(matches!(result[0].node, Node::Nil));
    }

    #[test]
    fn analyzes_string() {
        let result = parse("\"hello\"");
        assert!(matches!(&result[0].node, Node::String(s) if s == "hello"));
    }

    #[test]
    fn analyzes_keyword() {
        let result = parse(":foo");
        assert!(matches!(&result[0].node, Node::Keyword(s) if s == "foo"));
    }

    #[test]
    fn analyzes_symbol_as_var() {
        let result = parse("foo");
        assert!(matches!(result[0].node, Node::Var(_)));
    }

    // --- Compound ---

    #[test]
    fn analyzes_vector() {
        let result = parse("[1 2 3]");
        assert!(matches!(&result[0].node, Node::Vector(v) if v.len() == 3));
    }

    #[test]
    fn analyzes_map() {
        let result = parse("{:a 1}");
        assert!(matches!(&result[0].node, Node::Map(pairs) if pairs.len() == 1));
    }

    // --- if ---

    #[test]
    fn analyzes_if_with_else() {
        let result = parse("(if true 1 2)");
        assert!(matches!(&result[0].node, Node::If { _else: Some(_), .. }));
    }

    #[test]
    fn analyzes_if_without_else() {
        let result = parse("(if true 1)");
        assert!(matches!(&result[0].node, Node::If { _else: None, .. }));
    }

    #[test]
    fn error_if_too_few_args() {
        let err = parse_err("(if true)");
        assert!(matches!(err, AnalyzeError::InvalidArity { form: "if", .. }));
    }

    #[test]
    fn error_if_too_many_args() {
        let err = parse_err("(if true 1 2 3)");
        assert!(matches!(err, AnalyzeError::InvalidArity { form: "if", .. }));
    }

    // --- let ---

    #[test]
    fn analyzes_let() {
        let result = parse("(let [a 1] a)");
        assert!(matches!(
            &result[0].node,
            Node::Let { bindings, .. } if bindings.len() == 1
        ));
    }

    #[test]
    fn analyzes_let_multiple_bindings() {
        let result = parse("(let [a 1 b 2] a)");
        assert!(matches!(
            &result[0].node,
            Node::Let { bindings, .. } if bindings.len() == 2
        ));
    }

    #[test]
    fn error_let_bindings_not_vector() {
        let err = parse_err("(let (a 1) a)");
        assert!(matches!(err, AnalyzeError::InvalidBindings(_)));
    }

    #[test]
    fn error_let_odd_bindings() {
        let err = parse_err("(let [a 1 b] a)");
        assert!(matches!(err, AnalyzeError::OddBindings(_)));
    }

    #[test]
    fn error_let_non_symbol_key() {
        let err = parse_err("(let [1 2] a)");
        assert!(matches!(err, AnalyzeError::InvalidBindingKey(_)));
    }

    #[test]
    fn error_let_wrong_arity() {
        let err = parse_err("(let [a 1])");
        assert!(matches!(
            err,
            AnalyzeError::InvalidArity { form: "let", .. }
        ));
    }

    #[test]
    fn analyzes_fn() {
        let result = parse("(fn [a b] (+ a b))");
        assert!(matches!(
            &result[0].node,
            Node::Fn { params, .. } if params.len() == 2
        ));
    }

    #[test]
    fn analyzes_fn_no_params() {
        let result = parse("(fn [] 42)");
        assert!(matches!(
            &result[0].node,
            Node::Fn { params, .. } if params.is_empty()
        ));
    }

    #[test]
    fn error_fn_params_not_vector() {
        let err = parse_err("(fn (a b) body)");
        assert!(matches!(err, AnalyzeError::InvalidFnParams(_)));
    }

    #[test]
    fn error_fn_non_symbol_param() {
        let err = parse_err("(fn [a 1] body)");
        assert!(matches!(err, AnalyzeError::InvalidFnParams(_)));
    }

    #[test]
    fn error_fn_wrong_arity() {
        let err = parse_err("(fn [a b])");
        assert!(matches!(err, AnalyzeError::InvalidArity { form: "fn", .. }));
    }

    #[test]
    fn analyzes_def() {
        let result = parse("(def x 42)");
        assert!(matches!(&result[0].node, Node::Def { .. }));
    }

    #[test]
    fn error_def_non_symbol_name() {
        let err = parse_err("(def 1 42)");
        assert!(matches!(err, AnalyzeError::InvalidBindingKey(_)));
    }

    #[test]
    fn error_def_wrong_arity() {
        let err = parse_err("(def x)");
        assert!(matches!(
            err,
            AnalyzeError::InvalidArity { form: "def", .. }
        ));
    }

    #[test]
    fn analyzes_defn_as_def_fn() {
        let result = parse("(defn foo [a b] (+ a b))");
        assert!(matches!(
            &result[0].node,
            Node::Def { value, .. } if matches!(value.node, Node::Fn { .. })
        ));
    }

    #[test]
    fn error_defn_non_symbol_name() {
        let err = parse_err("(defn 1 [a b] body)");
        assert!(matches!(err, AnalyzeError::InvalidBindingKey(_)));
    }

    #[test]
    fn error_defn_params_not_vector() {
        let err = parse_err("(defn foo (a b) body)");
        assert!(matches!(err, AnalyzeError::InvalidFnParams(_)));
    }

    #[test]
    fn error_defn_wrong_arity() {
        let err = parse_err("(defn foo [a b])");
        assert!(matches!(
            err,
            AnalyzeError::InvalidArity { form: "defn", .. }
        ));
    }

    #[test]
    fn analyzes_call() {
        let result = parse("(foo 1 2)");
        assert!(matches!(
            &result[0].node,
            Node::Call { args, .. } if args.len() == 2
        ));
    }

    #[test]
    fn analyzes_call_no_args() {
        let result = parse("(foo)");
        assert!(matches!(
            &result[0].node,
            Node::Call { args, .. } if args.is_empty()
        ));
    }

    #[test]
    fn error_empty_list() {
        let err = parse_err("()");
        assert!(matches!(err, AnalyzeError::InvalidExpression(_)));
    }
}
