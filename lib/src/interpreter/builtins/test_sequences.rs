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

    // --- count ---

    #[test]
    fn count_list() {
        assert!(matches!(run("(count '(1 2 3))"), Value::Long(3)));
    }

    #[test]
    fn count_vector() {
        assert!(matches!(run("(count [1 2 3])"), Value::Long(3)));
    }

    #[test]
    fn count_set() {
        assert!(matches!(run("(count #{1 2})"), Value::Long(2)));
    }

    #[test]
    fn count_map() {
        assert!(matches!(run("(count {:a 1 :b 2})"), Value::Long(2)));
    }

    #[test]
    fn count_empty() {
        assert!(matches!(run("(count [])"), Value::Long(0)));
    }

    #[test]
    fn count_type_error() {
        assert!(matches!(run_err("(count 42)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn count_wrong_arity() {
        assert!(matches!(
            run_err("(count [1] [2])"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    // --- first ---

    #[test]
    fn first_non_empty_list() {
        assert!(matches!(run("(first '(10 20 30))"), Value::Long(10)));
    }

    #[test]
    fn first_empty_list_returns_nil() {
        assert!(matches!(run("(first '())"), Value::Nil));
    }

    #[test]
    fn first_vector() {
        assert!(matches!(run("(first [5 6 7])"), Value::Long(5)));
    }

    #[test]
    fn first_empty_vector_returns_nil() {
        assert!(matches!(run("(first [])"), Value::Nil));
    }

    #[test]
    fn first_map_returns_vector_pair() {
        assert!(matches!(run("(first {:a 1})"), Value::Vector(v) if v.len() == 2));
    }

    #[test]
    fn first_type_error() {
        assert!(matches!(run_err("(first 42)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn first_wrong_arity() {
        assert!(matches!(
            run_err("(first [1] [2])"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    // --- rest ---

    #[test]
    fn rest_list_returns_tail() {
        assert!(matches!(run("(rest '(1 2 3))"), Value::List(l) if l.len() == 2));
    }

    #[test]
    fn rest_empty_list_returns_empty() {
        assert!(matches!(run("(rest '())"), Value::List(l) if l.is_empty()));
    }

    #[test]
    fn rest_vector_returns_list() {
        assert!(matches!(run("(rest [1 2 3])"), Value::List(l) if l.len() == 2));
    }

    #[test]
    fn rest_empty_vector_returns_empty() {
        assert!(matches!(run("(rest [])"), Value::List(l) if l.is_empty()));
    }

    #[test]
    fn rest_map_returns_list_of_pairs() {
        assert!(matches!(run("(rest {:a 1 :b 2})"), Value::List(l) if l.len() == 1));
    }

    #[test]
    fn rest_wrong_arity() {
        assert!(matches!(
            run_err("(rest [1] [2])"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    // --- second ---

    #[test]
    fn second_two_plus_elements() {
        assert!(matches!(run("(second '(10 20 30))"), Value::Long(20)));
    }

    #[test]
    fn second_one_element_returns_nil() {
        assert!(matches!(run("(second '(1))"), Value::Nil));
    }

    #[test]
    fn second_empty_returns_nil() {
        assert!(matches!(run("(second '())"), Value::Nil));
    }

    #[test]
    fn second_vector_two_plus() {
        assert!(matches!(run("(second [10 20 30])"), Value::Long(20)));
    }

    #[test]
    fn second_map_single_entry_returns_nil() {
        assert!(matches!(run("(second {:a 1})"), Value::Nil));
    }

    #[test]
    fn second_wrong_arity() {
        assert!(matches!(
            run_err("(second [1] [2])"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    // --- last ---

    #[test]
    fn last_vector() {
        assert!(matches!(run("(last [1 2 3])"), Value::Long(3)));
    }

    #[test]
    fn last_empty_vector_returns_nil() {
        assert!(matches!(run("(last [])"), Value::Nil));
    }

    #[test]
    fn last_map_returns_vector_pair() {
        assert!(matches!(run("(last {:a 1 :b 2})"), Value::Vector(v) if v.len() == 2));
    }

    #[test]
    fn last_wrong_arity() {
        assert!(matches!(
            run_err("(last [1] [2])"),
            RuntimeError::WrongArity { expected: 1, .. }
        ));
    }

    // --- nth ---

    #[test]
    fn nth_list_valid_index() {
        assert!(matches!(run("(nth '(10 20 30) 1)"), Value::Long(20)));
    }

    #[test]
    fn nth_vector_valid_index() {
        assert!(matches!(run("(nth [10 20 30] 0)"), Value::Long(10)));
    }

    #[test]
    fn nth_out_of_bounds() {
        assert!(matches!(
            run_err("(nth '(1 2 3) 10)"),
            RuntimeError::IndexOutOfBounds { .. }
        ));
    }

    #[test]
    fn nth_negative_index_type_error() {
        assert!(matches!(run_err("(nth '(1 2 3) -1)"), RuntimeError::TypeError { .. }));
    }

    #[test]
    fn nth_map_unsupported_type() {
        assert!(matches!(
            run_err("(nth {:a 1} 0)"),
            RuntimeError::UnsupportedType { .. }
        ));
    }

    #[test]
    fn nth_wrong_arity() {
        assert!(matches!(
            run_err("(nth [1 2] 0 0)"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }

    // --- conj ---

    #[test]
    fn conj_vector_appends() {
        assert!(matches!(run("(conj [1 2] 3)"), Value::Vector(v) if v.len() == 3));
    }

    #[test]
    fn conj_vector_last_element() {
        assert!(matches!(run("(conj [1 2] 99)"), Value::Vector(v) if v[2] == Value::Long(99)));
    }

    #[test]
    fn conj_list_prepends() {
        assert!(matches!(run("(conj '(1 2) 0)"), Value::List(l) if l.first() == Some(&Value::Long(0))));
    }

    #[test]
    fn conj_set_adds_element() {
        assert!(matches!(run("(conj #{1 2} 3)"), Value::Set(s) if s.len() == 3));
    }

    #[test]
    fn conj_set_ignores_duplicate() {
        assert!(matches!(run("(conj #{1 2} 1)"), Value::Set(s) if s.len() == 2));
    }

    #[test]
    fn conj_set_normalizes_vector_to_list() {
        assert!(matches!(run("(conj #{} [1 2])"), Value::Set(s) if matches!(&s[0], Value::List(_))));
    }

    #[test]
    fn conj_set_deduplicates_vector_and_list() {
        assert!(matches!(run("(conj #{'(1 2)} [1 2])"), Value::Set(s) if s.len() == 1));
    }

    #[test]
    fn conj_map_with_pair() {
        assert!(matches!(run("(conj {:a 1} [:b 2])"), Value::Map(m) if m.len() == 2));
    }

    #[test]
    fn conj_map_with_map_merge() {
        assert!(matches!(run("(conj {:a 1} {:b 2})"), Value::Map(m) if m.len() == 2));
    }

    #[test]
    fn conj_wrong_arity() {
        assert!(matches!(
            run_err("(conj [1] 2 3)"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }

    // --- empty? ---

    #[test]
    fn empty_pred_empty_list() {
        assert!(matches!(run("(empty? '())"), Value::Bool(true)));
    }

    #[test]
    fn empty_pred_non_empty_list() {
        assert!(matches!(run("(empty? '(1))"), Value::Bool(false)));
    }

    #[test]
    fn empty_pred_empty_vector() {
        assert!(matches!(run("(empty? [])"), Value::Bool(true)));
    }

    #[test]
    fn empty_pred_non_empty_vector() {
        assert!(matches!(run("(empty? [1 2])"), Value::Bool(false)));
    }

    #[test]
    fn empty_pred_empty_map() {
        assert!(matches!(run("(empty? {})"), Value::Bool(true)));
    }

    #[test]
    fn empty_pred_empty_set() {
        assert!(matches!(run("(empty? #{})"), Value::Bool(true)));
    }

    #[test]
    fn empty_pred_type_error() {
        assert!(matches!(run_err("(empty? 42)"), RuntimeError::TypeError { .. }));
    }

    // --- cons ---

    #[test]
    fn cons_elem_and_list() {
        assert!(matches!(run("(cons 1 '(2 3))"), Value::List(l) if l.len() == 3));
    }

    #[test]
    fn cons_elem_and_list_first() {
        assert!(matches!(run("(cons 99 '(1 2))"), Value::List(l) if l.first() == Some(&Value::Long(99))));
    }

    #[test]
    fn cons_elem_and_vector_returns_list() {
        assert!(matches!(run("(cons 0 [1 2 3])"), Value::List(l) if l.len() == 4));
    }

    #[test]
    fn cons_elem_and_map_returns_list() {
        assert!(matches!(run("(cons 0 {:a 1})"), Value::List(l) if l.len() == 2));
    }

    #[test]
    fn cons_wrong_arity() {
        assert!(matches!(
            run_err("(cons 1 [2] [3])"),
            RuntimeError::WrongArity { expected: 2, .. }
        ));
    }
}
