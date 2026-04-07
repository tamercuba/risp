use std::collections::HashMap;

use crate::interpreter::Value;

#[allow(dead_code)]
pub struct Namespace {
    name: String,
    defs: HashMap<String, Value>,
    referred: Vec<String>,
}

impl Namespace {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            defs: Default::default(),
            referred: Default::default(),
        }
    }
    pub fn get(&self, name: &str) -> Option<Value> {
        self.defs.get(name).cloned()
    }

    pub fn global_names(&self) -> impl Iterator<Item = &str> {
        self.defs.keys().map(|s| s.as_str())
    }
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
            name: "core".to_string(),
            defs: Default::default(),
            referred: Default::default(),
        }
    }
}

pub struct NamespaceRegistry {
    namespaces: HashMap<String, Namespace>,
    pub current: String,
}

impl NamespaceRegistry {
    pub fn get(&self, name: &str) -> Option<Value> {
        self.get_in_ns(&self.current, name)
    }

    pub fn get_in_ns(&self, ns: &str, name: &str) -> Option<Value> {
        let ns = self.namespaces.get(ns)?;
        ns.get(name).or_else(|| {
            ns.referred
                .iter()
                .find_map(|referred_name| self.namespaces.get(referred_name)?.get(name))
        })
    }

    pub fn global_names(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for ns in self.namespaces.values() {
            result.extend(ns.global_names().map(|s| s.to_string()));
        }
        result
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.namespaces
            .entry(self.current.clone())
            .or_insert_with(|| Namespace::new(&self.current))
            .defs
            .insert(name.to_string(), value);
    }

    pub fn load(&mut self, ns_name: &str, values: Vec<(&'static str, Value)>) {
        let ns = self
            .namespaces
            .entry(ns_name.to_string())
            .or_insert_with(|| Namespace::new(ns_name));
        for (name, value) in values {
            ns.defs.insert(name.to_string(), value);
        }
    }

    pub fn create(&mut self, ns_name: &str, referred: Vec<&str>) {
        self.namespaces
            .entry(ns_name.to_string())
            .or_insert(Namespace {
                name: ns_name.to_string(),
                defs: Default::default(),
                referred: referred.into_iter().map(|s| s.to_string()).collect(),
            });
    }
}

impl Default for NamespaceRegistry {
    fn default() -> Self {
        Self {
            namespaces: Default::default(),
            current: "user".to_string(),
        }
    }
}
