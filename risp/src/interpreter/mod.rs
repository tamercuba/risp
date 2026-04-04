#![allow(dead_code)]
mod builtins;
mod env;
pub mod implementation;
#[cfg(test)]
mod test_interpreter;
mod value;

pub use builtins::math;
pub use env::Env;
pub use implementation::Interpreter;
