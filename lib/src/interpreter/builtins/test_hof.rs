#[cfg(test)]
mod tests {
    use crate::interpreter::implementation::Interpreter;
    use crate::interpreter::value::{RuntimeError, Value};

    fn run(source: &str) -> Value {
        Interpreter::new().run(source).unwrap()
    }

    fn run_err(source: &str) -> RuntimeError {
        Interpreter::new().run(source).unwrap_err()
    }

    // --- map ---

    #[test]
    fn map_vector_returns_vector() {
        assert!(matches!(run("(map (fn [x] x) [1 2 3])"), Value::Vector(v) if v.len() == 3));
    }

    #[test]
    fn map_list_returns_vector() {
        assert!(matches!(run("(map (fn [x] x) '(1 2 3))"), Value::Vector(v) if v.len() == 3));
    }

    #[test]
    fn map_applies_function() {
        assert!(matches!(run("(map (fn [x] (* x 2)) [1 2 3])"), Value::Vector(v) if *v == vec![Value::Long(2), Value::Long(4), Value::Long(6)]));
    }

    #[test]
    fn map_empty_collection() {
        assert!(matches!(run("(map (fn [x] x) [])"), Value::Vector(v) if v.is_empty()));
    }

    #[test]
    fn map_type_error_on_non_seq() {
        assert!(matches!(run_err("(map (fn [x] x) 42)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn map_wrong_arity() {
        assert!(matches!(
            run_err("(map (fn [x] x))"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }

    // --- filter ---

    #[test]
    fn filter_partial_predicate() {
        // Filter truthy values: nil and false are filtered out, numbers are truthy
        assert!(matches!(
            run("(filter (fn [x] x) [1 nil false 2])"),
            Value::List(l) if l.len() == 2
        ));
    }

    #[test]
    fn filter_rejects_all() {
        assert!(matches!(
            run("(filter (fn [x] false) [1 2 3])"),
            Value::List(l) if l.is_empty()
        ));
    }

    #[test]
    fn filter_accepts_all() {
        assert!(matches!(
            run("(filter (fn [x] true) [1 2 3])"),
            Value::List(l) if l.len() == 3
        ));
    }

    #[test]
    fn filter_nil_is_falsy() {
        assert!(matches!(
            run("(filter (fn [x] nil) [1 2 3])"),
            Value::List(l) if l.is_empty()
        ));
    }

    #[test]
    fn filter_type_error_on_non_seq() {
        assert!(matches!(run_err("(filter (fn [x] x) 42)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn filter_wrong_arity() {
        assert!(matches!(
            run_err("(filter (fn [x] x))"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }

    // --- reduce ---

    #[test]
    fn reduce_three_args() {
        assert!(matches!(run("(reduce + 0 [1 2 3])"), Value::Long(6)));
    }

    #[test]
    fn reduce_two_args() {
        assert!(matches!(run("(reduce + [1 2 3])"), Value::Long(6)));
    }

    #[test]
    fn reduce_empty_no_init_returns_nil() {
        assert!(matches!(run("(reduce + [])"), Value::Nil));
    }

    #[test]
    fn reduce_empty_with_init_returns_init() {
        assert!(matches!(run("(reduce + 42 [])"), Value::Long(42)));
    }

    #[test]
    fn reduce_single_element_no_init() {
        assert!(matches!(run("(reduce + [7])"), Value::Long(7)));
    }

    #[test]
    fn reduce_with_lambda() {
        assert!(matches!(
            run("(reduce (fn [a b] (* a b)) 1 [1 2 3 4])"),
            Value::Long(24)
        ));
    }

    // --- apply ---

    #[test]
    fn apply_list_as_last_arg() {
        assert!(matches!(run("(apply + '(1 2 3))"), Value::Long(6)));
    }

    #[test]
    fn apply_vector_as_last_arg() {
        assert!(matches!(run("(apply + [1 2 3])"), Value::Long(6)));
    }

    #[test]
    fn apply_extra_args_before_collection() {
        assert!(matches!(run("(apply + 1 2 [3 4])"), Value::Long(10)));
    }

    #[test]
    fn apply_single_arg_in_list() {
        assert!(matches!(run("(apply - [5])"), Value::Long(-5)));
    }

    #[test]
    fn apply_type_error_on_non_seq_last_arg() {
        assert!(matches!(run_err("(apply + 42)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn apply_wrong_arity_too_few() {
        assert!(matches!(
            run_err("(apply +)"),
            RuntimeError::WrongArity { .. }
        ));
    }
}
