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
            Token::LParen(Content::new((), 0, 1)),
            Token::LParen(Content::new((), 6, 2)),
            Token::Symbol(Content::new("let".to_string(), 7, 2)),
            Token::Symbol(Content::new("r".to_string(), 11, 2)),
            Token::Integer(Content::new(10, 13, 2)),
            Token::RParen(Content::new((), 15, 2)),
            Token::LParen(Content::new((), 17, 3)),
            Token::Symbol(Content::new("let".to_string(), 18, 3)),
            Token::Symbol(Content::new("pi".to_string(), 22, 3)),
            Token::Integer(Content::new(314, 25, 3)),
            Token::RParen(Content::new((), 28, 3)),
            Token::LParen(Content::new((), 30, 4)),
            Token::Symbol(Content::new("*".to_string(), 31, 4)),
            Token::Symbol(Content::new("pi".to_string(), 33, 4)),
            Token::LParen(Content::new((), 36, 4)),
            Token::Symbol(Content::new("*".to_string(), 37, 4)),
            Token::Symbol(Content::new("r".to_string(), 39, 4)),
            Token::Symbol(Content::new("r".to_string(), 41, 4)),
            Token::RParen(Content::new((), 42, 4)),
            Token::RParen(Content::new((), 43, 4)),
            Token::RParen(Content::new((), 44, 4))
        ]
    );
}
