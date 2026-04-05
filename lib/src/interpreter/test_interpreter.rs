#[cfg(test)]
mod tests {
    use crate::interpreter::{Interpreter, Value};

    fn run(source: &str) -> Value {
        Interpreter::new(false).run(source).unwrap()
    }

    fn run_err(source: &str) -> crate::interpreter::value::RuntimeError {
        Interpreter::new(false).run(source).unwrap_err()
    }

    #[test]
    fn eval_long() {
        assert!(matches!(run("42"), Value::Long(42)));
    }

    #[test]
    fn eval_negative_long() {
        assert!(matches!(run("-7"), Value::Long(-7)));
    }

    #[test]
    fn eval_double() {
        assert!(matches!(run("3.14"), Value::Double(v) if (v - 3.14).abs() < f64::EPSILON));
    }

    #[test]
    fn eval_bool_true() {
        assert!(matches!(run("true"), Value::Bool(true)));
    }

    #[test]
    fn eval_bool_false() {
        assert!(matches!(run("false"), Value::Bool(false)));
    }

    #[test]
    fn eval_nil() {
        assert!(matches!(run("nil"), Value::Nil));
    }

    #[test]
    fn eval_string() {
        assert!(matches!(run("\"hello\""), Value::String(s) if s == "hello"));
    }

    #[test]
    fn eval_keyword() {
        assert!(matches!(run(":foo"), Value::Keyword(s) if s == "foo"));
    }

    #[test]
    fn eval_if_true_branch() {
        assert!(matches!(run("(if true 1 2)"), Value::Long(1)));
    }

    #[test]
    fn eval_if_false_branch() {
        assert!(matches!(run("(if false 1 2)"), Value::Long(2)));
    }

    #[test]
    fn eval_if_nil_is_false() {
        assert!(matches!(run("(if nil 1 2)"), Value::Long(2)));
    }

    #[test]
    fn eval_if_no_else_returns_nil() {
        assert!(matches!(run("(if false 1)"), Value::Nil));
    }

    #[test]
    fn eval_if_type_error() {
        assert!(matches!(
            run_err("(if 42 1 2)"),
            crate::interpreter::value::RuntimeError::TypeError {
                expected: "bool",
                ..
            }
        ));
    }

    #[test]
    fn eval_let_basic() {
        assert!(matches!(run("(let [x 10] x)"), Value::Long(10)));
    }

    #[test]
    fn eval_let_multiple_bindings() {
        assert!(matches!(run("(let [x 1 y 2] y)"), Value::Long(2)));
    }

    #[test]
    fn eval_let_sequential_binding() {
        assert!(matches!(run("(let [x 3 y x] y)"), Value::Long(3)));
    }

    #[test]
    fn eval_let_scope_does_not_leak() {
        assert!(matches!(
            run_err("(let [x 1] nil) x"),
            crate::interpreter::value::RuntimeError::UndefinedVariable { .. }
        ));
    }

    #[test]
    fn eval_fn_returns_callable() {
        assert!(matches!(run("(fn [x] x)"), Value::Callable(_)));
    }

    #[test]
    fn eval_call_identity() {
        assert!(matches!(run("((fn [x] x) 42)"), Value::Long(42)));
    }

    #[test]
    fn eval_call_wrong_arity() {
        assert!(matches!(
            run_err("((fn [x] x) 1 2)"),
            crate::interpreter::value::RuntimeError::WrongArity {
                expected: 1,
                got: 2,
                ..
            }
        ));
    }

    #[test]
    fn eval_closure_captures_env() {
        assert!(matches!(run("(let [x 10] ((fn [] x)))"), Value::Long(10)));
    }

    #[test]
    fn eval_def_returns_nil() {
        assert!(matches!(run("(def x 42)"), Value::Nil));
    }

    #[test]
    fn eval_def_then_use() {
        assert!(matches!(run("(def x 99) x"), Value::Long(99)));
    }

    #[test]
    fn eval_defn_then_call() {
        assert!(matches!(
            run("(defn double [x] x) (double 7)"),
            Value::Long(7)
        ));
    }

    #[test]
    fn eval_vector_literal() {
        assert!(matches!(run("[1 2 3]"), Value::Vector(v) if v.len() == 3));
    }

    #[test]
    fn eval_empty_vector() {
        assert!(matches!(run("[]"), Value::Vector(v) if v.is_empty()));
    }

    #[test]
    fn eval_map_literal() {
        assert!(matches!(run("{:a 1}"), Value::Map(m) if m.len() == 1));
    }

    #[test]
    fn eval_empty_map() {
        assert!(matches!(run("{}"), Value::Map(m) if m.is_empty()));
    }

    #[test]
    fn eval_returns_last_form() {
        assert!(matches!(run("1 2 3"), Value::Long(3)));
    }

    #[test]
    fn eval_empty_source_returns_nil() {
        assert!(matches!(run(""), Value::Nil));
    }
}
