//! Contains the [`TreeErrorVariant`] trait implemented for all variants of [`TreeError`].
use crate::*;

/// trait for converting [`TreeError`] variants to [`DisplayedError`]s.
pub trait TreeErrorVariant<'a, R: pest::RuleType> {
    /// Get the [`ParsingContext`] used for creating an error.
    fn get_context(&self) -> Rc<ParsingContext>;
    /// Get the [`pest::iterators::Pair`] that failed to parse.
    fn get_pair(&self) -> &pest::iterators::Pair<'_, R>;
    /// Convert the error into a report.
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)>;
    /// Get the range of the error used for creating the report.
    /// This function is already implemented within the trait: don't implement this for the variants.
    fn get_range(&self) -> std::ops::Range<usize> {
        let mut tokens = self.get_pair().clone().tokens();
        let pest::Token::Start { rule, pos } = tokens.next().expect("invalid iterator") else {
            panic!("invalid token")
        };
        let start_pos = pos.pos();
        let pest::Token::End { rule, pos } = tokens.next().expect("invalid iterator") else {
            panic!("invalid token")
        };
        let end_pos = pos.pos();
        start_pos..end_pos
    }
    /// Convert the error to a [`DisplayedError`].
    /// There is no need to implement this function: it relies on [`TreeErrorVariant::to_report`].
    fn to_displayed_error(&self) -> DisplayedError<'a> {
        let rep = self.to_report();
        DisplayedError {
            report: rep,
            context: self.get_context(),
        }
    }
    /// Convert the error to a [`DisplayedTrace`].
    /// There is no need to impelment this function: it relies on [`TreeErrorVariant::to_displayed_error`].
    fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        let err = self.to_displayed_error();
        DisplayedTrace { reports: vec![err] }
    }
}
