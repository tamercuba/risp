#[cfg(test)]
use crate::{lexer::Token, parser::Object};

#[test]
fn test_parser_add() {
    let tokens = Token::tokenize("(+ 1 2)");
    let list = Object::from_tokens(tokens).unwrap();
    assert_eq!(
        list,
        Object::List(vec![
            Object::Symbol("+".to_string()),
            Object::Integer(1),
            Object::Integer(2)
        ])
    )
}

#[test]
fn test_parse_list() {
    let program = Token::tokenize("(1 2 3)");
    let list = Object::from_tokens(program).unwrap();
    assert_eq!(
        list,
        Object::List(vec![
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3)
        ])
    );
}

#[test]
fn test_area_of_a_circle() {
    let program = "(
        (let r 10)
        (let pi 314)
        (* pi (* r r))
    )";
    let tokens = Token::tokenize(program);

    let list = Object::from_tokens(tokens).unwrap();
    assert_eq!(
        list,
        Object::List(vec![
            Object::List(vec![
                Object::Symbol("let".to_string()),
                Object::Symbol("r".to_string()),
                Object::Integer(10)
            ]),
            Object::List(vec![
                Object::Symbol("let".to_string()),
                Object::Symbol("pi".to_string()),
                Object::Integer(314)
            ]),
            Object::List(vec![
                Object::Symbol("*".to_string()),
                Object::Symbol("pi".to_string()),
                Object::List(vec![
                    Object::Symbol("*".to_string()),
                    Object::Symbol("r".to_string()),
                    Object::Symbol("r".to_string())
                ])
            ])
        ])
    );
}

#[test]
fn test_parse_unbalanced_expression_whitout_closing() {
    let program = "(+ 1 2";
    let tokens = Token::tokenize(program);

    assert_eq!(tokens.len(), 4);

    let objs_list = Object::from_tokens(tokens.clone());

    assert!(objs_list.is_err());

    let err = objs_list.err().unwrap();

    assert_eq!(format!("{}", err), "0:1 Unmatched opening parenthesis");
}
#[test]
fn test_unbalanced_single_r_paren() {
    let program = ")";
    let tokens = Token::tokenize(program);

    assert_eq!(tokens.len(), 1);

    let objs_list = Object::from_tokens(tokens.clone());

    assert!(objs_list.is_err());

    let err = objs_list.err().unwrap();

    assert_eq!(format!("{}", err), "0:1 Unmatched closing parenthesis");
}
#[test]
fn test_parse_unbalanced_expressions_with_extra_closing() {
    let program = "(+ 1 2))";
    let tokens = Token::tokenize(program);

    assert_eq!(tokens.len(), 6);

    let objs_list = Object::from_tokens(tokens.clone());

    assert!(objs_list.is_err());

    let err = objs_list.err().unwrap();

    assert_eq!(format!("{}", err), "0:8 Unmatched closing parenthesis");
}

#[test]
fn test_list_dislay_trait() {
    let list = Object::List(vec![
        Object::Integer(1),
        Object::Integer(2),
        Object::Integer(3),
    ]);
    assert_eq!(format!("{}", list), "(1 2 3)");
}
