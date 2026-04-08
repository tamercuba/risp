#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Span};
    use crate::parser::{Expr, ExprKind, ParseError, Parser};

    fn span() -> Span {
        Span { lo: 0, hi: 0 }
    }

    fn expr(kind: ExprKind) -> Expr {
        Expr { kind, span: span() }
    }

    fn parse(input: &str) -> Vec<Expr> {
        Parser::parse(Lexer::tokenize(input)).unwrap()
    }

    fn parse_err(input: &str) -> ParseError {
        Parser::parse(Lexer::tokenize(input)).unwrap_err()
    }

    #[test]
    fn parses_long() {
        let result = parse("42");
        assert_eq!(result[0].kind, ExprKind::Long(42));
    }

    #[test]
    fn parses_negative_long() {
        let result = parse("-7");
        assert_eq!(result[0].kind, ExprKind::Long(-7));
    }

    #[test]
    fn parses_double() {
        let result = parse("3.14");
        assert_eq!(result[0].kind, ExprKind::Double(3.14));
    }

    #[test]
    fn parses_string() {
        let result = parse("\"hello\"");
        assert_eq!(result[0].kind, ExprKind::String("hello".to_string()));
    }

    #[test]
    fn parses_keyword() {
        let result = parse(":foo");
        assert_eq!(result[0].kind, ExprKind::Keyword("foo".to_string()));
    }

    #[test]
    fn parses_symbol() {
        let result = parse("foo");
        assert_eq!(result[0].kind, ExprKind::Symbol("foo".to_string()));
    }

    #[test]
    fn parses_true() {
        let result = parse("true");
        assert_eq!(result[0].kind, ExprKind::Bool(true));
    }

    #[test]
    fn parses_false() {
        let result = parse("false");
        assert_eq!(result[0].kind, ExprKind::Bool(false));
    }

    #[test]
    fn parses_nil() {
        let result = parse("nil");
        assert_eq!(result[0].kind, ExprKind::Nil);
    }

    #[test]
    fn parses_empty_list() {
        let result = parse("()");
        assert_eq!(result[0].kind, ExprKind::List(vec![]));
    }

    #[test]
    fn parses_list() {
        let result = parse("(+ 1 2)");
        assert_eq!(
            result[0].kind,
            ExprKind::List(vec![
                expr(ExprKind::Symbol("+".to_string())),
                expr(ExprKind::Long(1)),
                expr(ExprKind::Long(2)),
            ])
        );
    }

    #[test]
    fn parses_empty_vector() {
        let result = parse("[]");
        assert_eq!(result[0].kind, ExprKind::Vector(vec![]));
    }

    #[test]
    fn parses_vector() {
        let result = parse("[1 2 3]");
        assert_eq!(
            result[0].kind,
            ExprKind::Vector(vec![
                expr(ExprKind::Long(1)),
                expr(ExprKind::Long(2)),
                expr(ExprKind::Long(3)),
            ])
        );
    }

    #[test]
    fn parses_empty_map() {
        let result = parse("{}");
        assert_eq!(result[0].kind, ExprKind::Map(vec![]));
    }

    #[test]
    fn parses_map() {
        let result = parse("{:a 1}");
        assert_eq!(
            result[0].kind,
            ExprKind::Map(vec![(
                expr(ExprKind::Keyword("a".to_string())),
                expr(ExprKind::Long(1)),
            )])
        );
    }

    #[test]
    fn parses_nested() {
        let result = parse("(+ [1 2] 3)");
        assert_eq!(
            result[0].kind,
            ExprKind::List(vec![
                expr(ExprKind::Symbol("+".to_string())),
                expr(ExprKind::Vector(vec![
                    expr(ExprKind::Long(1)),
                    expr(ExprKind::Long(2)),
                ])),
                expr(ExprKind::Long(3)),
            ])
        );
    }

    #[test]
    fn parses_multiple_forms() {
        let result = parse("1 2 3");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].kind, ExprKind::Long(1));
        assert_eq!(result[1].kind, ExprKind::Long(2));
        assert_eq!(result[2].kind, ExprKind::Long(3));
    }

    #[test]
    fn error_unmatched_open_paren() {
        let err = parse_err("(+ 1 2");
        assert!(matches!(err, ParseError::UnmatchedOpen(_)));
    }

    #[test]
    fn error_unmatched_close_paren() {
        let err = parse_err(")");
        assert!(matches!(err, ParseError::UnmatchedClose(')', _)));
    }

    #[test]
    fn error_mismatched_paren_bracket() {
        let err = parse_err("(+ 1 2]");
        assert!(matches!(
            err,
            ParseError::MismatchedDelimiter {
                expected: ')',
                found: ']',
                ..
            }
        ));
    }

    #[test]
    fn error_mismatched_bracket_paren() {
        let err = parse_err("[1 2)");
        assert!(matches!(
            err,
            ParseError::MismatchedDelimiter {
                expected: ']',
                found: ')',
                ..
            }
        ));
    }

    #[test]
    fn error_odd_map_elements() {
        let err = parse_err("{:a}");
        assert!(matches!(err, ParseError::OddMapElements(_)));
    }

    #[test]
    fn parses_qualified_symbol() {
        let result = parse("risp.core/map");
        assert_eq!(
            result[0].kind,
            ExprKind::QualifiedSymbol { ns: "risp.core".into(), name: "map".into() }
        );
    }

    #[test]
    fn parses_simple_qualified_symbol() {
        let result = parse("foo/bar");
        assert_eq!(
            result[0].kind,
            ExprKind::QualifiedSymbol { ns: "foo".into(), name: "bar".into() }
        );
    }

    #[test]
    fn parses_slash_alone_as_symbol() {
        let result = parse("/");
        assert_eq!(result[0].kind, ExprKind::Symbol("/".into()));
    }

    #[test]
    fn parses_malformed_trailing_slash_as_symbol() {
        let result = parse("foo/");
        assert_eq!(result[0].kind, ExprKind::Symbol("foo/".into()));
    }

    #[test]
    fn qualified_symbol_inside_list() {
        let result = parse("(risp.core/+ 1 2)");
        match &result[0].kind {
            ExprKind::List(elems) => assert_eq!(
                elems[0].kind,
                ExprKind::QualifiedSymbol { ns: "risp.core".into(), name: "+".into() }
            ),
            _ => panic!("expected list"),
        }
    }
}
