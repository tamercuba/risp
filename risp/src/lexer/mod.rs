pub(crate) mod implementation;
#[cfg(test)]
mod test_lexer;
#[cfg(test)]
mod test_token;
pub(crate) mod token;
pub use implementation::Lexer;
pub use token::{Span, Token};
