#[cfg(test)]
use crate::{ env::Env, parser::Object };

#[test]
fn test_env_new() {
    let env = Env::new();
    assert!(env.borrow().vars().is_empty());
}

#[test]
fn test_add_var() {
    let env = Env::new();
    env.borrow_mut().set("x", Object::Integer(10));
    assert_eq!(env.borrow().get("x"), Some(Object::Integer(10)));
}

#[test]
fn test_add_var_with_parent() {
    let parent = Env::new();
    parent.borrow_mut().set("x", Object::Integer(10));

    let env = Env::new_scope(parent.clone());
    env.borrow_mut().set("y", Object::Integer(20));

    assert!(env.borrow().vars().get("x").is_none(), "X var only existis in parent environment");
    assert_eq!(
        env.borrow().get("x"),
        Some(Object::Integer(10)),
        "X is acessible using get because functions can access vars defines outside of their scope"
    );
    assert_eq!(env.borrow().get("y"), Some(Object::Integer(20)));
}

#[test]
fn test_remove_scope_without_parent() {
    let env = Env::new();
    env.borrow_mut().set("x", Object::Integer(10));
    env.borrow_mut().set("y", Object::Integer(20));

    env.borrow_mut().remove_scope();

    assert!(env.borrow().vars().is_empty());
}
