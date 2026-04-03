#![allow(dead_code)]

use std::{cell::Cell, collections::HashMap, rc::Rc};

pub struct Scope<'a> {
    bindings: HashMap<String, u32>,
    parent: Option<&'a Scope<'a>>,
    next_id: Rc<Cell<u32>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
            next_id: Rc::new(Cell::new(0)),
        }
    }

    pub fn enter_scope(&'a self) -> Scope<'a> {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
            next_id: Rc::clone(&self.next_id),
        }
    }

    pub fn bind(&mut self, name: String) -> u32 {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        self.bindings.insert(name, id);
        id
    }

    pub fn get_by_name(&self, name: &str) -> Option<u32> {
        match self.bindings.get(name) {
            Some(id) => Some(*id),
            None => self.parent.and_then(|p| p.get_by_name(name)),
        }
    }
}
