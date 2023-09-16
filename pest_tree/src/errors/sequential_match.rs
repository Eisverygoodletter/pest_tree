//! When matching a struct sequentially, a member failed to match.
use super::super::*;
use super::*;
use ariadne::Fmt;
/// Refer to [`TreeError::SequentialMatchError`].
#[derive(Debug, Clone)]
pub struct SequentialMatchError<'a, R: pest::RuleType> {
    /// Pair (representing the whole struct) that failed to match.
    pub pair: pest::iterators::Pair<'a, R>,
    /// parsing context.
    pub context: Rc<ParsingContext>,
    /// The cause of the failure. Can be caused by overall requirements not being fulfilled or some member field
    /// not matching.
    pub cause: Box<TreeError<'a, R>>,
}

impl<'a, R: pest::RuleType + 'a> TreeErrorVariant<'a, R> for SequentialMatchError<'a, R> {
    fn get_pair(&self) -> &pest::iterators::Pair<'_, R> {
        &self.pair
    }
    fn get_context(&self) -> Rc<ParsingContext> {
        self.context.clone()
    }
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        let range = self.get_range();
        ariadne::Report::build(ariadne::ReportKind::Error, self.context.filename.clone(), 0)
            .with_label(
                ariadne::Label::new((self.context.filename.clone(), range))
                    .with_message("Failed to match due to failing to match a member"),
            )
            .with_code(4)
            .with_message("Did not match Sequentially")
            .finish()
    }
    fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        let err = self.to_displayed_error();
        let mut disp: DisplayedTrace<'a> = DisplayedTrace { reports: vec![err] };
        let mut other_disp: DisplayedTrace<'a> = (*self.cause).clone().to_displayed_trace();
        disp.add_cause(&mut other_disp);
        disp
    }
}

// todo: find a way to create a sequential match error
