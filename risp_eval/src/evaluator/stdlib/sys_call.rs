use crate::parser::Object;
use std::fmt;

type SysCall = fn(&Vec<Object>) -> Result<Object, String>;

#[derive(Clone)]
pub struct SysCallWrapper {
    name: String,
    func: Box<SysCall>,
}

impl fmt::Debug for SysCallWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<std::{}>", self.name)
    }
}

impl PartialEq for SysCallWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl SysCallWrapper {
    pub fn new(name: &str, func: SysCall) -> Self {
        Self {
            name: name.to_string(),
            func: Box::new(func),
        }
    }

    pub fn run(&self, args: &Vec<Object>) -> Result<Object, String> {
        (self.func)(args)
    }
}
