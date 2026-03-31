pub(crate) mod implementation;
#[cfg(test)]
mod test_lexer;
pub(crate) mod token;
pub use implementation::Lexer;
pub use token::{Span, Token};
