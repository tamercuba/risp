#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Span};
    use crate::parser::cst::{Expr, ExprKind};
    use crate::parser::parser_impl::ParseError;
    use crate::parser::Parser;

    fn span() -> Span {
        Span { lo: 0, hi: 0 }
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
                Expr::new(ExprKind::Symbol("+".to_string()), span()),
                Expr::new(ExprKind::Long(1), span()),
                Expr::new(ExprKind::Long(2), span()),
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
                Expr::new(ExprKind::Long(1), span()),
                Expr::new(ExprKind::Long(2), span()),
                Expr::new(ExprKind::Long(3), span()),
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
                Expr::new(ExprKind::Keyword("a".to_string()), span()),
                Expr::new(ExprKind::Long(1), span()),
            )])
        );
    }

    #[test]
    fn parses_nested() {
        let result = parse("(+ [1 2] 3)");
        assert_eq!(
            result[0].kind,
            ExprKind::List(vec![
                Expr::new(ExprKind::Symbol("+".to_string()), span()),
                Expr::new(
                    ExprKind::Vector(vec![
                        Expr::new(ExprKind::Long(1), span()),
                        Expr::new(ExprKind::Long(2), span()),
                    ]),
                    span()
                ),
                Expr::new(ExprKind::Long(3), span()),
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
}
