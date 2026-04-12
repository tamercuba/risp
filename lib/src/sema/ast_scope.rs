use std::collections::HashMap;

pub type LocalId = u32;

pub struct Scope<'a> {
    bindings: HashMap<String, LocalId>,
    parent: Option<&'a Scope<'a>>,
    next_id: LocalId,
}

impl<'a> Scope<'a> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
            next_id: 0,
        }
    }

    pub fn enter_scope(&'a self) -> Scope<'a> {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
            next_id: self.next_id,
        }
    }

    pub fn enter_fn_scope(&'a self) -> Scope<'a> {
        Self {
            bindings: HashMap::new(),
            parent: Some(self),
            next_id: self.next_id,
        }
    }

    pub fn bind(&mut self, name: String) -> LocalId {
        let id = self.next_id;
        self.next_id += 1;
        self.bindings.insert(name, id);
        id
    }

    pub fn get_by_name(&self, name: &str) -> Option<LocalId> {
        match self.bindings.get(name) {
            Some(id) => Some(*id),
            None => self.parent.and_then(|p| p.get_by_name(name)),
        }
    }
}
