#[cfg(test)]
mod tests {
    use crate::lexer::{Content, Lexer, Span, Token};

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
                Token::Long(Content::new(1, span(3, 4))),
                Token::Long(Content::new(2, span(5, 6))),
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
                Token::Long(Content::new(10, span(8, 10))),
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
    fn tokenizes_multiline_program_starting_with_comment() {
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
    fn tokenizes_string_literal() {
        let tokens = Lexer::tokenize("\"hello world\"");
        assert_eq!(
            tokens,
            vec![Token::String(Content::new(
                "hello world".to_string(),
                span(0, 13)
            ))]
        );
    }

    #[test]
    fn tokenizes_string_in_expression() {
        let tokens = Lexer::tokenize("(println \"hi\")");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("println".to_string(), span(1, 8))),
                Token::String(Content::new("hi".to_string(), span(9, 13))),
                Token::RParen(Content::new((), span(13, 14))),
            ]
        );
    }

    #[test]
    fn tokenizes_keyword() {
        let tokens = Lexer::tokenize(":foo");
        assert_eq!(
            tokens,
            vec![Token::Keyword(Content::new("foo".to_string(), span(0, 4)))]
        );
    }

    #[test]
    fn tokenizes_keyword_in_expression() {
        let tokens = Lexer::tokenize("(assoc m :key 1)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("assoc".to_string(), span(1, 6))),
                Token::Symbol(Content::new("m".to_string(), span(7, 8))),
                Token::Keyword(Content::new("key".to_string(), span(9, 13))),
                Token::Long(Content::new(1, span(14, 15))),
                Token::RParen(Content::new((), span(15, 16))),
            ]
        );
    }

    #[test]
    fn tokenizes_vector_literal() {
        let tokens = Lexer::tokenize("[1 2 3]");
        assert_eq!(
            tokens,
            vec![
                Token::LBracket(Content::new((), span(0, 1))),
                Token::Long(Content::new(1, span(1, 2))),
                Token::Long(Content::new(2, span(3, 4))),
                Token::Long(Content::new(3, span(5, 6))),
                Token::RBracket(Content::new((), span(6, 7))),
            ]
        );
    }

    #[test]
    fn tokenizes_map_literal() {
        let tokens = Lexer::tokenize("{:a 1}");
        assert_eq!(
            tokens,
            vec![
                Token::LBrace(Content::new((), span(0, 1))),
                Token::Keyword(Content::new("a".to_string(), span(1, 3))),
                Token::Long(Content::new(1, span(4, 5))),
                Token::RBrace(Content::new((), span(5, 6))),
            ]
        );
    }

    #[test]
    fn tokenizes_double_literal() {
        let tokens = Lexer::tokenize("3.14");
        assert_eq!(tokens, vec![Token::Double(Content::new(3.14, span(0, 4)))]);
    }

    #[test]
    fn tokenizes_double_in_expression() {
        let tokens = Lexer::tokenize("(+ 1 2.5)");
        assert_eq!(
            tokens,
            vec![
                Token::LParen(Content::new((), span(0, 1))),
                Token::Symbol(Content::new("+".to_string(), span(1, 2))),
                Token::Long(Content::new(1, span(3, 4))),
                Token::Double(Content::new(2.5, span(5, 8))),
                Token::RParen(Content::new((), span(8, 9))),
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
}
