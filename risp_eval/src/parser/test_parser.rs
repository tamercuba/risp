#[cfg(test)]
use crate::{ parser::Object, lexer::Token };

#[test]
fn test_parser_add() {
    let tokens = Token::tokenize("(+ 1 2)").unwrap();
    let list = Object::from_tokens(tokens).unwrap();
    assert_eq!(
        list,
        Object::List(vec![Object::Symbol("+".to_string()), Object::Integer(1), Object::Integer(2)])
    )
}

#[test]
fn test_area_of_a_circle() {
    let program = "(
        (define r 10)
        (define pi 314)
        (* pi (* r r))
    )";
    let tokens = Token::tokenize(program).unwrap();
    let list = Object::from_tokens(tokens).unwrap();
    assert_eq!(
        list,
        Object::List(
            vec![
                Object::List(
                    vec![
                        Object::Symbol("define".to_string()),
                        Object::Symbol("r".to_string()),
                        Object::Integer(10)
                    ]
                ),
                Object::List(
                    vec![
                        Object::Symbol("define".to_string()),
                        Object::Symbol("pi".to_string()),
                        Object::Integer(314)
                    ]
                ),
                Object::List(
                    vec![
                        Object::Symbol("*".to_string()),
                        Object::Symbol("pi".to_string()),
                        Object::List(
                            vec![
                                Object::Symbol("*".to_string()),
                                Object::Symbol("r".to_string()),
                                Object::Symbol("r".to_string())
                            ]
                        )
                    ]
                )
            ]
        )
    );
}
