#[cfg(test)]
mod tests {
    use crate::interpreter::implementation::Interpreter;
    use crate::interpreter::value::Value;

    fn run(source: &str) -> Value {
        Interpreter::new().run(source).unwrap()
    }

    #[test]
    fn list_builtin_creates_list() {
        assert!(matches!(run("(list 1 2 3)"), Value::List(l) if l.len() == 3));
    }

    #[test]
    fn list_builtin_empty() {
        assert!(matches!(run("(list)"), Value::List(l) if l.is_empty()));
    }

    #[test]
    fn list_builtin_first_element() {
        assert!(matches!(run("(list 10 20)"), Value::List(l) if l.first() == Some(&Value::Long(10))));
    }

    #[test]
    fn vector_builtin_creates_vector() {
        assert!(matches!(run("(vector 1 2 3)"), Value::Vector(v) if v.len() == 3));
    }

    #[test]
    fn vector_builtin_empty() {
        assert!(matches!(run("(vector)"), Value::Vector(v) if v.is_empty()));
    }

    #[test]
    fn vector_builtin_elements() {
        assert!(matches!(
            run("(vector 1 2 3)"),
            Value::Vector(v) if v == vec![Value::Long(1), Value::Long(2), Value::Long(3)]
        ));
    }

    #[test]
    fn hash_map_builtin_creates_map() {
        assert!(matches!(run("(hash-map :a 1 :b 2)"), Value::Map(m) if m.len() == 2));
    }

    #[test]
    fn hash_map_builtin_empty() {
        assert!(matches!(run("(hash-map)"), Value::Map(m) if m.is_empty()));
    }

    #[test]
    fn hash_map_builtin_keys_and_values() {
        let result = run("(hash-map :a 1)");
        match result {
            Value::Map(m) => {
                assert_eq!(m.len(), 1);
                assert_eq!(m[0].0, Value::Keyword("a".into()));
                assert_eq!(m[0].1, Value::Long(1));
            }
            _ => panic!("expected Map"),
        }
    }
}
