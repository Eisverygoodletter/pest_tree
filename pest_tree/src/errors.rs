//! [`TreeError`]s and its variants used for error reporting and pretty printing.
use super::*;
use ariadne;

pub mod tree_error_variant;
pub use tree_error_variant::*;

pub mod direct_match;
pub use direct_match::*;
pub mod string_conversion;
pub use string_conversion::*;
pub mod box_conversion;
pub use box_conversion::*;
pub mod displayed;
pub use displayed::*;
pub mod sequential_match;
pub use sequential_match::*;
pub mod pair_count;
pub use pair_count::*;
/// An error emitted by when parsing. 
/// 
/// For pretty-printing, you can call [`TreeError::eprint`]
/// 
/// ```no_run
/// // the contents are "b" and the rule is Rule::b. The PestParser won't detect any error, but
/// // it won't be successfully parsed into struct A.
/// let parsed_iter = PestParser::parse(Rule::b, ctx.contents.clone()).unwrap();
/// // ::parse returns a Pairs. This Pairs only contains 1 Pair, which we will be using.
/// let pair = parsed_iter.next().unwrap();
/// // struct A implements PestTree
/// // ctx was defined earlier
/// let err = A::from_pair(pair, ctx.clone()).unwrap_err();
/// // pretty print error to console
/// err.eprint();
/// ```
#[derive(Debug, Clone)]
pub enum TreeError<'a, T: pest::RuleType> {
    /// Pair doesn't match the rule specified.
    DirectMatchError(DirectMatchError<'a, T>),
    /// Failed to automatically convert from a [`str`] to the specified type. This is most likely an internal error.
    StringConversionError(StringConversionError<'a, T>),
    /// Failed to enclose some value within a [`Box`]. This is most likely an internal error.
    BoxConversionError(BoxConversionError<'a, T>),
    /// When matching a struct sequentially, a member failed to match.
    SequentialMatchError(SequentialMatchError<'a, T>),
    /// When matching a struct sequentially, the number of [`pest::iterators::Pair`]s in the
    /// [`pest::iterators::Pairs`] did not match the number of fields in the struct.
    PairCountError(PairCountError<'a, T>),
}
impl<'a, T: pest::RuleType + 'a> TreeError<'a, T> {
    /// Pretty print the error to the console.
    /// 
    /// Internally this function converts itself into a [`DisplayedTrace`] and calls
    /// [`DisplayedTrace::eprint`]
    /// 
    /// ```no_run
    /// // ... get a pair and a context ...
    /// let tree_err = A::from_pair(pair, ctx.clone()).unwrap_err();
    /// tree_err.eprint();
    /// ```
    pub fn eprint(&self) {
        let trace: DisplayedTrace = (*self).clone().to_displayed_trace();
        trace.eprint();
    }
    /// Convert itself into an [`ariadne::Report`]. Consider using [`TreeError::to_displayed_trace`] instead.
    pub fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        match self {
            TreeError::DirectMatchError(v) => v.to_report(),
            TreeError::StringConversionError(v) => v.to_report(),
            TreeError::BoxConversionError(v) => v.to_report(),
            TreeError::SequentialMatchError(v) => v.to_report(),
            TreeError::PairCountError(v) => v.to_report(),
        }
    }
    /// Convert itself to a [`DisplayedTrace`]. If you only want the error printed to the console, use
    /// [`TreeError::eprint`]
    pub fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        match self.clone() {
            TreeError::DirectMatchError(v) => v.clone().to_displayed_trace(),
            TreeError::StringConversionError(v) => v.clone().to_displayed_trace(),
            TreeError::BoxConversionError(v) => v.clone().to_displayed_trace(),
            TreeError::SequentialMatchError(v) => v.to_displayed_trace(),
            TreeError::PairCountError(v) => v.to_displayed_trace(),
        }
    }
}
