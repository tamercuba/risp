#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Span, Token};
    use crate::lexer::token::Content;

    fn span(lo: u32, hi: u32) -> Span {
        Span { lo, hi }
    }

    #[test]
    fn tokenizes_arithmetic_expression() {
        let tokens = Lexer::tokenize("(+ 1 2)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("+".to_string(), span(1, 2))),
                Token::Integer(Content::new(1, span(3, 4))),
                Token::Integer(Content::new(2, span(5, 6))),
                Token::RParen(Content::new((), span(6, 7))),
            ]
        );
    }

    #[test]
    fn tokenizes_consecutive_delimiters() {
        let tokens = Lexer::tokenize("(let((x 10)))");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("let".to_string(), span(1, 4))),
                Token::LParen(Content::new((), span(4, 5))),
                Token::LParen(Content::new((), span(5, 6))),
                Token::Symbol(Content::new("x".to_string(), span(6, 7))),
                Token::Integer(Content::new(10, span(8, 10))),
                Token::RParen(Content::new((), span(10, 11))),
                Token::RParen(Content::new((), span(11, 12))),
                Token::RParen(Content::new((), span(12, 13))),
            ]
        );
    }

    #[test]
    fn tokenizes_multiline_program() {
        let tokens = Lexer::tokenize("(foo\n bar)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("foo".to_string(), span(1, 4))),
                Token::Symbol(Content::new("bar".to_string(), span(6, 9))),
                Token::RParen(Content::new((), span(9, 10))),
            ]
        );
    }

    #[test]
    fn tokenizes_multiline_program_with_comment() {
        let tokens = Lexer::tokenize("(foo)\n(bar);(bla)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("foo".to_string(), span(1, 4))),
                Token::RParen(Content::new((), span(4, 5))),
                Token::LParen(Content::new((), span(6, 7))),
                Token::Symbol(Content::new("bar".to_string(), span(7, 10))),
                Token::RParen(Content::new((), span(10, 11))),
            ]
        );
    }

    #[test]
    fn tokenizes_multiline_program_startign_with_comment() {
        let tokens = Lexer::tokenize(";1234567\n(foo)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(10, 11))),
                Token::Symbol(Content::new("foo".to_string(), span(11, 14))),
                Token::RParen(Content::new((), span(14, 15))),
            ]
        );
    }

    #[test]
    fn empty_input_produces_no_tokens() {
        assert_eq!(Lexer::tokenize(""), vec![]);
    }

    #[test]
    fn only_whitespace_produces_no_tokens() {
        assert_eq!(Lexer::tokenize("   \t\n  "), vec![]);
    }

    #[test]
    fn span_covers_token_at_end_of_input() {
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
        let tokens = Lexer::tokenize(" 314 ");
        assert_eq!(tokens.len(), 1);
        let s = match &tokens[0] {
            Token::Integer(c) => &c.span,
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
            format!("{}", Token::Integer(Content::new(42, span(3, 5)))),
            "3..5 Int(42)"
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
            format!("{:?}", Token::Integer(Content::new(42, span(3, 5)))),
            "3..5 Int(42)"
        );
    }
}
