#[cfg(test)]
use crate::{ parser::Object, evaluator::Evaluator };

#[test]
fn test_eval_add_func() {
    let program = "(+ 1 2)";
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::Integer(3));
}

#[test]
fn test_eval_division_by_zero() {
    let program = "(/ 1 0)";
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval(program);

    assert!(result.is_err());

    let result = result.err().unwrap();

    assert_eq!(result, "Division by zero");
}

#[test]
fn test_eval_with_var() {
    let program = "
    (
     (define r 10)
     (define pi 314)
     (* pi (* r r))
    )
    ";
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(31400)]));
}

#[test]
fn test_eval_lambda_into_define() {
    let program = "
    (
     (define square (lambda (x) (* x x)))
     (square 10)
    )
    ";
    let mut evaluator = Evaluator::new();

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
    let mut evaluator = Evaluator::new();

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
    let mut evaluator = Evaluator::new();

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(10)]));
}
