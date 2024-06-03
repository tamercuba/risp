#[cfg(test)]
use crate::{ evaluator::stdlib::funcs::*, parser::Object };

#[test]
fn test_to_str() {
    let objs = vec![Object::Integer(10)];
    let result = to_str(&objs);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::String("10".to_string()));
}

#[test]
fn test_to_str_without_args() {
    let objs = vec![];
    let result = to_str(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Expected 1 argument, found 0");
}

#[test]
fn test_to_str_with_multiple_args() {
    let objs = vec![Object::Integer(10), Object::Integer(20)];
    let result = to_str(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Expected 1 argument, found 2");
}

#[test]
fn test_concat_str() {
    let objs = vec![
        Object::String("Hello".to_string()),
        Object::String(",".to_string()),
        Object::String(" ".to_string()),
        Object::String("World".to_string()),
        Object::String("!".to_string())
    ];
    let result = concat_str(&objs);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::String("Hello, World!".to_string()));
}

#[test]
fn test_concat_str_without_args() {
    let objs = vec![];
    let result = concat_str(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Expected at least 1 argument, found 0");
}

#[test]
fn test_take_first() {
    let objs = vec![Object::List(vec![Object::Integer(10), Object::Integer(20)])];
    let result = list_take_first(&objs);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::Integer(10));
}

#[test]
fn test_take_first_without_args() {
    let objs = vec![];
    let result = list_take_first(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Expected 1 argument, found 0");
}
