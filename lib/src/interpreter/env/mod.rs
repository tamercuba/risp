mod namespace;

use self::namespace::NamespaceRegistry;

use super::Value;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct Env {
    locals: HashMap<u32, Value>,
    registry: Rc<RefCell<NamespaceRegistry>>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent.clone()),
            registry: parent.borrow().registry.clone(),
            ..Default::default()
        }
    }

    pub fn get_local(&self, id: u32) -> Option<Value> {
        self.locals
            .get(&id)
            .cloned()
            .or_else(|| self.parent.as_ref()?.borrow().get_local(id))
    }

    pub fn set_local(&mut self, id: u32, value: Value) -> Option<()> {
        self.locals.insert(id, value);
        None
    }

    pub fn get_global(&self, name: &str) -> Option<Value> {
        let current_ns = self.registry.borrow();
        current_ns.get(name)
    }

    pub fn get_in_ns(&self, ns: &str, name: &str) -> Option<Value> {
        self.registry.borrow().get_in_ns(ns, name)
    }

    pub fn public_names(&self) -> Vec<String> {
        self.registry.borrow().public_names()
    }

    pub fn set_global(&mut self, name: &str, value: Value) {
        match self.parent.as_ref() {
            Some(parent) => parent.borrow_mut().set_global(name, value),
            None => {
                self.registry.borrow_mut().set(name, value);
            }
        }
    }

    pub fn load_builtins(&self, ns_name: &str, values: Vec<(&'static str, Value)>) {
        self.registry.borrow_mut().load(ns_name, values);
    }

    pub fn create_ns(&self, ns_name: &str, referred: Vec<&str>) {
        self.registry.borrow_mut().create(ns_name, referred);
    }

    pub fn get_current_namespace(&self) -> Rc<str> {
        self.registry.borrow().current.clone()
    }

    pub fn set_current_namespace(&self, ns: &str) {
        self.registry.borrow_mut().current = ns.into();
    }
}
