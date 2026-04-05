#[cfg(test)]
mod tests {
    use crate::interpreter::implementation::Interpreter;
    use crate::interpreter::value::{RuntimeError, Value};

    fn run(source: &str) -> Value {
        Interpreter::new(true).run(source).unwrap()
    }

    fn run_err(source: &str) -> RuntimeError {
        Interpreter::new(true).run(source).unwrap_err()
    }

    fn long(n: i64) -> Value {
        Value::Long(n)
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-10
    }

    #[test]
    fn plus_integers() {
        assert!(matches!(run("(+ 1 2 3)"), Value::Long(6)));
    }

    #[test]
    fn plus_empty() {
        assert!(matches!(run("(+)"), Value::Long(0)));
    }

    #[test]
    fn plus_single() {
        assert!(matches!(run("(+ 42)"), Value::Long(42)));
    }

    #[test]
    fn plus_promotes_to_double() {
        assert!(matches!(run("(+ 1 2.0)"), Value::Double(v) if approx(v, 3.0)));
    }

    #[test]
    fn plus_type_error() {
        assert!(matches!(
            run_err("(+ 1 \"foo\")"),
            RuntimeError::TypeError { .. }
        ));
    }

    #[test]
    fn minus_two_args() {
        assert!(matches!(run("(- 10 3)"), Value::Long(7)));
    }

    #[test]
    fn minus_negate() {
        assert!(matches!(run("(- 5)"), Value::Long(-5)));
    }

    #[test]
    fn minus_multiple_args() {
        assert!(matches!(run("(- 10 3 2)"), Value::Long(5)));
    }

    #[test]
    fn minus_promotes_to_double() {
        assert!(matches!(run("(- 10.0 3)"), Value::Double(v) if approx(v, 7.0)));
    }

    #[test]
    fn minus_no_args_error() {
        assert!(matches!(
            run_err("(-)"),
            RuntimeError::WrongArity {
                expected: 1,
                got: 0,
                ..
            }
        ));
    }

    #[test]
    fn minus_type_error() {
        assert!(matches!(
            run_err("(- \"foo\" 1)"),
            RuntimeError::TypeError { .. }
        ));
    }

    #[test]
    fn times_integers() {
        assert!(matches!(run("(* 1 2 3 4 5)"), Value::Long(120)));
    }

    #[test]
    fn times_empty() {
        assert!(matches!(run("(*)"), Value::Long(1)));
    }

    #[test]
    fn times_single() {
        assert!(matches!(run("(* 7)"), Value::Long(7)));
    }

    #[test]
    fn times_promotes_to_double() {
        assert!(matches!(run("(* 2 1.5)"), Value::Double(v) if approx(v, 3.0)));
    }

    #[test]
    fn times_type_error() {
        assert!(matches!(
            run_err("(* 2 \"foo\")"),
            RuntimeError::TypeError { .. }
        ));
    }

    #[test]
    fn divide_integers() {
        assert!(matches!(run("(/ 10 2)"), Value::Long(5)));
    }

    #[test]
    fn divide_returns_double() {
        assert!(matches!(run("(/ 10.0 4.0)"), Value::Double(v) if approx(v, 2.5)));
    }

    #[test]
    fn divide_long_by_double() {
        assert!(matches!(run("(/ 10 4.0)"), Value::Double(v) if approx(v, 2.5)));
    }

    #[test]
    fn divide_by_zero_long() {
        assert!(matches!(
            run_err("(/ 10 0)"),
            RuntimeError::DivisionByZero(_)
        ));
    }

    #[test]
    fn divide_by_zero_double() {
        assert!(matches!(
            run_err("(/ 10 0.0)"),
            RuntimeError::DivisionByZero(_)
        ));
    }

    #[test]
    fn divide_wrong_arity() {
        assert!(matches!(
            run_err("(/ 10 2 5)"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }

    #[test]
    fn divide_type_error() {
        assert!(matches!(
            run_err("(/ \"foo\" 2)"),
            RuntimeError::TypeError { .. }
        ));
    }
}
