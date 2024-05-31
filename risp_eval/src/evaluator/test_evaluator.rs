#[cfg(test)]
use crate::{ env::Env, parser::Object, evaluator::Evaluator };

#[test]
fn test_eval_add_func() {
    let program = "(+ 1 2)";
    let env = Env::new();
    let mut evaluator = Evaluator::new(env);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::Integer(3));
}

#[test]
fn test_eval_division_by_zero() {
    let program = "(/ 1 0)";
    let env = Env::new();
    let mut evaluator = Evaluator::new(env);

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
    let env = Env::new();
    let mut evaluator = Evaluator::new(env);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(31400)]));
}

#[test]
fn test_eval_with_lambda() {
    let program = "
    (
     (define square (lambda (x) (* x x)))
     (square 10)
    )
    ";
    let env = Env::new();
    let mut evaluator = Evaluator::new(env);

    let result = evaluator.eval(program);

    assert!(result.is_ok());

    let result = result.unwrap();

    assert_eq!(result, Object::List(vec![Object::Integer(100)]));
}