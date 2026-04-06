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

    #[test]
    fn eq_equal_longs() {
        assert!(matches!(run("(= 1 1)"), Value::Bool(true)));
    }

    #[test]
    fn eq_different_longs() {
        assert!(matches!(run("(= 1 2)"), Value::Bool(false)));
    }

    #[test]
    fn eq_equal_strings() {
        assert!(matches!(run(r#"(= "a" "a")"#), Value::Bool(true)));
    }

    #[test]
    fn eq_equal_keywords() {
        assert!(matches!(run("(= :foo :foo)"), Value::Bool(true)));
    }

    #[test]
    fn eq_nil_nil() {
        assert!(matches!(run("(= nil nil)"), Value::Bool(true)));
    }

    #[test]
    fn eq_bool_true() {
        assert!(matches!(run("(= true true)"), Value::Bool(true)));
    }

    #[test]
    fn eq_multiple_all_equal() {
        assert!(matches!(run("(= 1 1 1)"), Value::Bool(true)));
    }

    #[test]
    fn eq_multiple_last_differs() {
        assert!(matches!(run("(= 1 1 2)"), Value::Bool(false)));
    }

    #[test]
    fn eq_list_and_vector_equal() {
        assert!(matches!(run("(= '(1 2 3) [1 2 3])"), Value::Bool(true)));
    }

    #[test]
    fn eq_vector_and_list_equal() {
        assert!(matches!(run("(= [1 2 3] '(1 2 3))"), Value::Bool(true)));
    }

    #[test]
    fn eq_list_and_vector_different_values() {
        assert!(matches!(run("(= '(1 2) [1 3])"), Value::Bool(false)));
    }

    #[test]
    fn eq_list_and_vector_different_lengths() {
        assert!(matches!(run("(= '(1 2) [1 2 3])"), Value::Bool(false)));
    }

    #[test]
    fn eq_empty_list_and_empty_vector() {
        assert!(matches!(run("(= '() [])"), Value::Bool(true)));
    }

    #[test]
    fn eq_set_not_equal_to_list() {
        assert!(matches!(run("(= #{1 2} '(1 2))"), Value::Bool(false)));
    }

    #[test]
    fn eq_equal_sets() {
        assert!(matches!(run("(= #{1 2} #{1 2})"), Value::Bool(true)));
    }

    #[test]
    fn eq_equal_maps() {
        assert!(matches!(run("(= {:a 1} {:a 1})"), Value::Bool(true)));
    }

    #[test]
    fn eq_one_arg_returns_true() {
        assert!(matches!(run("(= 1)"), Value::Bool(true)));
    }

    #[test]
    fn eq_wrong_arity_zero_args() {
        assert!(matches!(
            run_err("(=)"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    #[test]
    fn neq_different() {
        assert!(matches!(run("(not= 1 2)"), Value::Bool(true)));
    }

    #[test]
    fn neq_equal() {
        assert!(matches!(run("(not= 1 1)"), Value::Bool(false)));
    }

    #[test]
    fn neq_multiple_all_equal() {
        assert!(matches!(run("(not= 1 1 1)"), Value::Bool(false)));
    }

    #[test]
    fn neq_multiple_one_differs() {
        assert!(matches!(run("(not= 1 1 2)"), Value::Bool(true)));
    }

    #[test]
    fn gt_true() {
        assert!(matches!(run("(> 3 2)"), Value::Bool(true)));
    }

    #[test]
    fn gt_false() {
        assert!(matches!(run("(> 2 3)"), Value::Bool(false)));
    }

    #[test]
    fn gt_equal_is_false() {
        assert!(matches!(run("(> 2 2)"), Value::Bool(false)));
    }

    #[test]
    fn gt_chain_true() {
        assert!(matches!(run("(> 5 3 1)"), Value::Bool(true)));
    }

    #[test]
    fn gt_chain_false() {
        assert!(matches!(run("(> 5 3 3)"), Value::Bool(false)));
    }

    #[test]
    fn gt_long_and_double() {
        assert!(matches!(run("(> 3 1.5)"), Value::Bool(true)));
    }

    #[test]
    fn gt_single_arg_returns_true() {
        assert!(matches!(run("(> 1)"), Value::Bool(true)));
    }

    #[test]
    fn gt_wrong_arity_zero_args() {
        assert!(matches!(
            run_err("(>)"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    #[test]
    fn gt_type_error_on_string() {
        assert!(matches!(
            run_err(r#"(> "a" "b")"#),
            RuntimeError::TypeError { .. }
        ));
    }

    #[test]
    fn gt_double_vs_long() {
        assert!(matches!(run("(> 3.5 2)"), Value::Bool(true)));
        assert!(matches!(run("(> 1.5 2)"), Value::Bool(false)));
    }

    #[test]
    fn gt_double_vs_double() {
        assert!(matches!(run("(> 2.5 1.5)"), Value::Bool(true)));
        assert!(matches!(run("(> 1.5 2.5)"), Value::Bool(false)));
    }

    #[test]
    fn lt_true() {
        assert!(matches!(run("(< 1 2)"), Value::Bool(true)));
    }

    #[test]
    fn lt_false() {
        assert!(matches!(run("(< 2 1)"), Value::Bool(false)));
    }

    #[test]
    fn lt_equal_is_false() {
        assert!(matches!(run("(< 3 3)"), Value::Bool(false)));
    }

    #[test]
    fn lt_chain_true() {
        assert!(matches!(run("(< 1 2 3)"), Value::Bool(true)));
    }

    #[test]
    fn lt_chain_false() {
        assert!(matches!(run("(< 1 3 2)"), Value::Bool(false)));
    }

    #[test]
    fn lt_single_arg_returns_true() {
        assert!(matches!(run("(< 5)"), Value::Bool(true)));
    }

    #[test]
    fn lt_wrong_arity_zero_args() {
        assert!(matches!(run_err("(<)"), RuntimeError::WrongArity { .. }));
    }

    #[test]
    fn lt_type_error_on_string() {
        assert!(matches!(
            run_err(r#"(< "a" "b")"#),
            RuntimeError::TypeError { .. }
        ));
    }

    #[test]
    fn lt_long_and_double() {
        assert!(matches!(run("(< 1 1.5)"), Value::Bool(true)));
        assert!(matches!(run("(< 1.5 1)"), Value::Bool(false)));
    }

    // le
    #[test]
    fn le_true() {
        assert!(matches!(run("(<= 1 2)"), Value::Bool(true)));
    }

    #[test]
    fn le_equal_is_true() {
        assert!(matches!(run("(<= 3 3)"), Value::Bool(true)));
    }

    #[test]
    fn le_false() {
        assert!(matches!(run("(<= 3 2)"), Value::Bool(false)));
    }

    #[test]
    fn le_chain_true() {
        assert!(matches!(run("(<= 1 2 2 3)"), Value::Bool(true)));
    }

    #[test]
    fn le_single_arg_returns_true() {
        assert!(matches!(run("(<= 5)"), Value::Bool(true)));
    }

    #[test]
    fn le_wrong_arity_zero_args() {
        assert!(matches!(run_err("(<=)"), RuntimeError::WrongArity { .. }));
    }

    #[test]
    fn ge_true() {
        assert!(matches!(run("(>= 2 1)"), Value::Bool(true)));
    }

    #[test]
    fn ge_equal_is_true() {
        assert!(matches!(run("(>= 3 3)"), Value::Bool(true)));
    }

    #[test]
    fn ge_false() {
        assert!(matches!(run("(>= 1 2)"), Value::Bool(false)));
    }

    #[test]
    fn ge_chain_true() {
        assert!(matches!(run("(>= 3 2 2 1)"), Value::Bool(true)));
    }

    #[test]
    fn ge_single_arg_returns_true() {
        assert!(matches!(run("(>= 5)"), Value::Bool(true)));
    }

    #[test]
    fn ge_wrong_arity_zero_args() {
        assert!(matches!(run_err("(>=)"), RuntimeError::WrongArity { .. }));
    }
}
