#[cfg(test)]
use crate::{ evaluator::stdlib::funcs::*, parser::Object, evaluator::SysCallWrapper };

#[test]
fn test_str_to_str_() {
    let objs = vec![Object::String("Hello".to_string())];
    let result = to_str(&objs);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::String("Hello".to_string()));
}

#[test]
fn test_int_to_str() {
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
fn test_invalid_arg_to_str() {
    let objs = vec![Object::Bool(true)];
    let result = to_str(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Invalid argument 'true', cannot convert to str");
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
fn test_concat_str_with_invalid_args() {
    let objs = vec![Object::String("Hello".to_string()), Object::Integer(10)];
    let result = concat_str(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Invalid value 10, it isnt a str");
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

#[test]
fn test_take_first_with_empty_list() {
    let objs = vec![Object::List(vec![])];
    let result = list_take_first(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Cannot take first element of empty list");
}

#[test]
fn test_take_first_not_list_as_arg() {
    let objs = vec![Object::Integer(10)];
    let result = list_take_first(&objs);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "10 is not a List");
}

#[test]
fn test_sys_call_wrapper_debug_trait() {
    let wrapper = SysCallWrapper::new("test", |args| { Ok(Object::String(format!("{:?}", args))) });
    let expected_output = "<std::test>";

    assert_eq!(format!("{:?}", wrapper), expected_output);
}

#[test]
fn test_sys_call_wrapper_eq_trait() {
    let wrapper1 = SysCallWrapper::new("test", |args| {
        Ok(Object::String(format!("{:?}", args)))
    });
    let wrapper2 = SysCallWrapper::new("test", |args| {
        Ok(Object::String(format!("{:?}", args)))
    });

    assert_eq!(wrapper1, wrapper2);
}

#[test]
fn test_print_ln() {
    let objs = vec![Object::String("Hello".to_string()), Object::String("World".to_string())];
    let result = print_ln(&objs);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::Void);
}
