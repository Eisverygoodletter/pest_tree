//! An easy and simple way to convert your dynamic [pest] trees into an AST built from structs and enums.
//! 
//! This crate is centered around the [`pest_tree_derive::PestTree`] derive macro.
//! The macro uses the `pest_tree` attribute macro to help derive the trait 
//! [`pest_tree_derive::PestTree`] for the relevant struct/enum.
#![doc = include_str!("../../README.md")]
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

/// Contains information about the original source file/string.
/// 
/// This struct is often passed around in an [`std::rc::Rc`] and is used to show [`TreeError`] sources.
/// 
/// ```
/// // to initialize ctx
/// use pest_tree::ParsingContext;
/// use std::rc::Rc;
/// let ctx = Rc::new(ParsingContext {
///     filename: "mystatement.lang".to_string(),
///     contents: "let a = 3;".to_string()
/// });
/// ```
/// `ctx` can then be passed into [`PestTree::with_pair`].
#[derive(Debug, Clone)]
pub struct ParsingContext {
    /// Filename. This will be shown on top of errors.
    pub filename: String,
    /// Contents of the file being parsed. The [`TreeError`]s formed require this for proper pretty printing.
    pub contents: String,
}
impl ParsingContext {
    /// Initialize a [`ParsingContext`] from [`str`]s.
    pub fn new_file(filename: &str, contents: &str) -> Self {
        Self {
            filename: filename.to_string(),
            contents: contents.to_string(),
        }
    }
}

/// Implemented for types that can be parsed from [`pest::iterators::Pair`].
/// 
/// Contains the function [`PestTree::with_pair`], which a [`pest::iterators::Pair`] to be converted and a
/// [`Rc<ParsingContext>`] that specifies the original source. This trait is normally implemented by the 
/// [`pest_tree_derive::PestTree`] derive macro, but you can also implement it yourself.
pub trait PestTree<R: pest::RuleType> {
    /// Convert a [`pest::iterators::Pair`] to the implementor.
    /// 
    /// A context is also specified so that error reporting via [`ariadne`] is possible.
    /// Returns `Result<T, TreeError>`. [`TreeError`] can be pretty printed.
    /// 
    /// ```no_run
    /// let parsed_iter = PestParser::parse(Rule::a, ctx.contents.clone()).unwrap();
    /// // ::parse returns a Pairs. This Pairs only contains 1 Pair, which we will be using.
    /// let pair = parsed_iter.next().unwrap();
    /// // struct A implements PestTree
    /// // ctx was defined earlier
    /// let a: A = A::from_pair(pair, ctx.clone()).unwrap();
    /// // now the pair has been parsed into `a`.
    /// ```
    fn with_pair(
        pair: pest::iterators::Pair<'_, R>,
        context: Rc<ParsingContext>,
    ) -> Result<Self, TreeError<R>>
    where
        Self: Sized;
}
