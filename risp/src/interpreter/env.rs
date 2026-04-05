use super::Value;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct Env {
    locals: HashMap<u32, Value>,
    globals: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn with_parent(parent: Rc<RefCell<Env>>) -> Self {
        Self {
            parent: Some(parent),
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
        self.globals
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref()?.borrow().get_global(name))
    }

    pub fn set_global(&mut self, name: &str, value: Value) {
        match self.parent.as_ref() {
            Some(parent) => parent.borrow_mut().set_global(name, value),
            None => {
                self.globals.insert(name.to_string(), value);
            }
        }
    }
}
