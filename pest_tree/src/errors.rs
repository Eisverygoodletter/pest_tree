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

/**
 * An error emitted by when parsing. For pretty-printing , consider using [`DisplayedError`] and [`DisplayedTrace`].
 */
#[derive(Debug, Clone)]
pub enum TreeError<'a, T: pest::RuleType> {
    DirectMatchError(DirectMatchError<'a, T>),
    StringConversionError(StringConversionError<'a, T>),
    BoxConversionError(BoxConversionError<'a, T>),
    SequentialMatchError(SequentialMatchError<'a, T>),
}
impl<'a, T: pest::RuleType + 'a> TreeError<'a, T> {
    pub fn eprint(&self) {
        let trace: DisplayedTrace = (*self).clone().to_displayed_trace();
        trace.eprint();
    }
    pub fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        match self {
            TreeError::DirectMatchError(v) => v.to_report(),
            TreeError::StringConversionError(v) => v.to_report(),
            TreeError::BoxConversionError(v) => v.to_report(),
            TreeError::SequentialMatchError(v) => v.to_report(),
        }
    }
    pub fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        match self.clone() {
            TreeError::DirectMatchError(v) => v.clone().to_displayed_trace(),
            TreeError::StringConversionError(v) => v.clone().to_displayed_trace(),
            TreeError::BoxConversionError(v) => v.clone().to_displayed_trace(),
            TreeError::SequentialMatchError(v) => v.to_displayed_trace(),
        }
    }
}
