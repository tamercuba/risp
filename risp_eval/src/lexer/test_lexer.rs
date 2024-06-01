#[cfg(test)]
use super::implementation::Token;

#[test]
fn test_add() {
    let tokens = Token::tokenize("(+ 1 2)").unwrap_or(vec![]);
    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::Symbol("+".to_string()),
            Token::Integer(1),
            Token::Integer(2),
            Token::RParen
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
    let tokens = Token::tokenize(program).unwrap_or(vec![]);
    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::LParen,
            Token::Symbol("let".to_string()),
            Token::Symbol("r".to_string()),
            Token::Integer(10),
            Token::RParen,
            Token::LParen,
            Token::Symbol("let".to_string()),
            Token::Symbol("pi".to_string()),
            Token::Integer(314),
            Token::RParen,
            Token::LParen,
            Token::Symbol("*".to_string()),
            Token::Symbol("pi".to_string()),
            Token::LParen,
            Token::Symbol("*".to_string()),
            Token::Symbol("r".to_string()),
            Token::Symbol("r".to_string()),
            Token::RParen,
            Token::RParen,
            Token::RParen
        ]
    );
}
