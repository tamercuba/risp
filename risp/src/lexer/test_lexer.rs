#[cfg(test)]
use super::implementation::{ Token, Content };

#[test]
fn test_add() {
    let tokens = Token::tokenize("(+ 1 2)");
    assert_eq!(
        tokens,
        vec![
            Token::LParen(Content::new((), 0, 1)),
            Token::Symbol(Content::new("+".to_string(), 1, 1)),
            Token::Integer(Content::new(1, 3, 1)),
            Token::Integer(Content::new(2, 5, 1)),
            Token::RParen(Content::new((), 6, 1))
        ]
    )
}

#[test]
fn test_area_of_a_circle() {
    let program = "
    (
     (let r 10)
     (let pi 314)
     (* pi (* r r))
    )
    ";
    let tokens = Token::tokenize(program);
    assert_eq!(
        tokens,
        vec![
            Token::LParen(Content::new((), 5, 1)),
            Token::LParen(Content::new((), 6, 2)),
            Token::Symbol(Content::new("let".to_string(), 7, 2)),
            Token::Symbol(Content::new("r".to_string(), 11, 2)),
            Token::Integer(Content::new(10, 13, 2)),
            Token::RParen(Content::new((), 15, 2)),
            Token::LParen(Content::new((), 6, 3)),
            Token::Symbol(Content::new("let".to_string(), 7, 3)),
            Token::Symbol(Content::new("pi".to_string(), 1, 3)),
            Token::Integer(Content::new(314, 14, 3)),
            Token::RParen(Content::new((), 17, 3)),
            Token::LParen(Content::new((), 6, 4)),
            Token::Symbol(Content::new("*".to_string(), 7, 4)),
            Token::Symbol(Content::new("pi".to_string(), 9, 4)),
            Token::LParen(Content::new((), 12, 4)),
            Token::Symbol(Content::new("*".to_string(), 13, 4)),
            Token::Symbol(Content::new("r".to_string(), 15, 4)),
            Token::Symbol(Content::new("r".to_string(), 17, 4)),
            Token::RParen(Content::new((), 18, 4)),
            Token::RParen(Content::new((), 19, 4)),
            Token::RParen(Content::new((), 5, 5))
        ]
    );
}

#[test]
fn test_let_withou_space() {
    let program = "(let((x 10)))";
    let tokens = Token::tokenize(program);

    assert_eq!(
        tokens,
        vec![
            Token::LParen(Content::new((), 1, 0)),
            Token::Symbol(Content::new("let".to_string(), 2, 0)),
            Token::LParen(Content::new((), 5, 0)),
            Token::LParen(Content::new((), 6, 0)),
            Token::Symbol(Content::new("x".to_string(), 7, 0)),
            Token::Integer(Content::new(10, 9, 0)),
            Token::RParen(Content::new((), 11, 0)),
            Token::RParen(Content::new((), 12, 0)),
            Token::RParen(Content::new((), 13, 0))
        ]
    );
}

#[test]
fn test_debug_token_debug_trait() {
    assert_eq!(format!("{:?}", Token::LParen(Content::new((), 0, 0))), "0:0 LParen");
    assert_eq!(
        format!("{:?}", Token::Symbol(Content::new("let".to_string(), 0, 0))),
        "0:0 Symbol(let)"
    );
    assert_eq!(format!("{:?}", Token::Integer(Content::new(10, 0, 0))), "0:0 Int(10)");
    assert_eq!(format!("{:?}", Token::RParen(Content::new((), 0, 0))), "0:0 RParen");
}

#[test]
fn test_debug_token_display_trait() {
    assert_eq!(format!("{}", Token::LParen(Content::new((), 0, 0))), "0:0 LParen");
    assert_eq!(
        format!("{}", Token::Symbol(Content::new("let".to_string(), 0, 0))),
        "0:0 Symbol(let)"
    );
    assert_eq!(format!("{}", Token::Integer(Content::new(10, 0, 0))), "0:0 Int(10)");
    assert_eq!(format!("{}", Token::RParen(Content::new((), 0, 0))), "0:0 RParen");
}
