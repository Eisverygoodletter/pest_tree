#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]


use std::{fmt::Display, marker::PhantomData, rc::Rc};
pub mod errors;
pub use ariadne;
pub use errors::*;
pub mod auto;
pub use auto::*;
use std::rc;

/// Contains information such the original source string
#[derive(Debug, Clone)]
pub struct ParsingContext {
    pub filename: String,
    pub contents: String,
}
impl ParsingContext {
    pub fn new_file(filename: &str, contents: &str) -> Self {
        Self {
            filename: filename.to_string(),
            contents: contents.to_string(),
        }
    }
}

/**
 * C
 */
pub trait PestTree<R: pest::RuleType> {
    fn from_pest(
        pair: pest::iterators::Pair<'_, R>,
        context: Rc<ParsingContext>,
    ) -> Result<Self, TreeError<R>>
    where
        Self: Sized;
}
