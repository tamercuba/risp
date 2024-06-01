#[cfg(test)]
use crate::{ parser::Object, evaluator::Evaluator };

#[test]
fn test_eval_tokens_with_error() {
    let program = "(+ 1 2";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_err());

    let result = result.err().unwrap();
    assert_eq!(result, "Unmatched closing parenthesis");
}

#[test]
fn test_eval_add_func() {
    let program = "(+ 1 2)";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::Integer(3));
}

#[test]
fn test_eval_division_by_zero() {
    let program = "(/ 1 0)";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Division by zero");
}

#[test]
fn test_eval_with_var() {
    let program = "
    (
     (let r 10)
     (let pi 314)
     (* pi (* r r))
    )
    ";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(31400)]));
}

#[test]
fn test_eval_lambda_into_let() {
    let program = "
    (
     (let square (lambda (x) (* x x)))
     (square 10)
    )
    ";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(100)]));
}

#[test]
fn test_eval_function_definition() {
    let program = "
    (
     (defun square (x) (* x x))
     (square 10)
    )
    ";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(100)]));
}

#[test]
fn test_eval_nested_lambda() {
    let program = "
    (
        ((lambda () ((lambda () ((lambda (x y) (+ x y)) 3 7)))))
    )
    ";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(10)]));
}

#[test]
fn test_eval_sys_call() {
    let program = "(str 10)";
    let mut evaluator = Evaluator::new(true);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::String("10".to_string()));
}

#[test]
fn test_eval_sys_call_not_binded() {
    let program = "(str 10)";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Function not defined: str");
}

#[test]
fn test_func_call_doesnt_exist() {
    let program = "(foo 10)";
    let mut evaluator = Evaluator::new(false);

    let result = evaluator.eval(program);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "Function not defined: foo");
}

#[test]
fn test_eval_with_boolean() {
    let program1 = "(let x true)";
    let program2 = "(if x 10 20)";
    let mut evaluator = Evaluator::new(false);

    let _ = evaluator.eval(program1).unwrap();
    let result = evaluator.eval(program2);

    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(result, Object::Integer(10));
}
