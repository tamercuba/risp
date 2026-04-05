#[cfg(test)]
mod tests {
    use crate::lexer::{Content, Span, Token};

    fn span(lo: u32, hi: u32) -> Span {
        Span { lo, hi }
    }

    #[test]
    fn span_covers_token_at_end_of_input() {
        use crate::lexer::Lexer;
        let tokens = Lexer::tokenize("hello");
        assert_eq!(tokens.len(), 1);
        let s = match &tokens[0] {
            Token::Symbol(c) => &c.span,
            t => panic!("expected Symbol, got {:?}", t),
        };
        assert_eq!(s.lo, 0);
        assert_eq!(s.hi, 5);
    }

    #[test]
    fn span_of_integer_is_exact() {
        use crate::lexer::Lexer;
        let tokens = Lexer::tokenize(" 314 ");
        assert_eq!(tokens.len(), 1);
        let s = match &tokens[0] {
            Token::Long(c) => &c.span,
            t => panic!("expected Integer, got {:?}", t),
        };
        assert_eq!(s.lo, 1);
        assert_eq!(s.hi, 4);
    }

    #[test]
    fn token_display_format() {
        assert_eq!(
            format!("{}", Token::LParen(Content::new((), span(0, 1)))),
            "0..1 LParen"
        );
        assert_eq!(
            format!("{}", Token::RParen(Content::new((), span(6, 7)))),
            "6..7 RParen"
        );
        assert_eq!(
            format!(
                "{}",
                Token::Symbol(Content::new("+".to_string(), span(1, 2)))
            ),
            "1..2 Symbol(+)"
        );
        assert_eq!(
            format!("{}", Token::Long(Content::new(42, span(3, 5)))),
            "3..5 Long(42)"
        );
        assert_eq!(
            format!(
                "{}",
                Token::Keyword(Content::new("foo".to_string(), span(0, 4)))
            ),
            "0..4 Keyword(foo)"
        );
        assert_eq!(
            format!(
                "{}",
                Token::String(Content::new("hello".to_string(), span(0, 7)))
            ),
            "0..7 String(hello)"
        );
        assert_eq!(
            format!("{}", Token::LBracket(Content::new((), span(0, 1)))),
            "0..1 LBracket"
        );
        assert_eq!(
            format!("{}", Token::RBracket(Content::new((), span(1, 2)))),
            "1..2 RBracket"
        );
        assert_eq!(
            format!("{}", Token::LBrace(Content::new((), span(0, 1)))),
            "0..1 LBrace"
        );
        assert_eq!(
            format!("{}", Token::RBrace(Content::new((), span(1, 2)))),
            "1..2 RBrace"
        );
    }

    #[test]
    fn token_debug_format() {
        assert_eq!(
            format!("{:?}", Token::LParen(Content::new((), span(0, 1)))),
            "0..1 LParen"
        );
        assert_eq!(
            format!("{:?}", Token::RParen(Content::new((), span(6, 7)))),
            "6..7 RParen"
        );
        assert_eq!(
            format!(
                "{:?}",
                Token::Symbol(Content::new("+".to_string(), span(1, 2)))
            ),
            "1..2 Symbol(+)"
        );
        assert_eq!(
            format!("{:?}", Token::Long(Content::new(42, span(3, 5)))),
            "3..5 Long(42)"
        );
        assert_eq!(
            format!(
                "{:?}",
                Token::Keyword(Content::new("foo".to_string(), span(0, 4)))
            ),
            "0..4 Keyword(foo)"
        );
        assert_eq!(
            format!(
                "{:?}",
                Token::String(Content::new("hello".to_string(), span(0, 7)))
            ),
            "0..7 String(hello)"
        );
        assert_eq!(
            format!("{:?}", Token::LBracket(Content::new((), span(0, 1)))),
            "0..1 LBracket"
        );
        assert_eq!(
            format!("{:?}", Token::RBracket(Content::new((), span(1, 2)))),
            "1..2 RBracket"
        );
        assert_eq!(
            format!("{:?}", Token::LBrace(Content::new((), span(0, 1)))),
            "0..1 LBrace"
        );
        assert_eq!(
            format!("{:?}", Token::RBrace(Content::new((), span(1, 2)))),
            "1..2 RBrace"
        );
    }
}
