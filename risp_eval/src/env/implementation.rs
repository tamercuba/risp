use crate::parser::Object;
use std::{ cell::RefCell, collections::HashMap, rc::Rc };

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_scope(parent: Rc<RefCell<Env>>) -> Self {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn remove_scope(&mut self) {
        if let Some(parent) = self.parent.clone() {
            let parent = parent.borrow();
            self.vars = parent.vars.clone();
            self.parent = parent.parent.clone();
        } else {
            self.vars.clear();
            self.parent = None;
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|parent| parent.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: &str, value: Object) {
        self.vars.insert(name.to_string(), value);
    }
}
