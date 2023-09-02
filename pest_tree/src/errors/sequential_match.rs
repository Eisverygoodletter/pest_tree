use super::*;
use super::super::*;
use ariadne::Fmt;
/// Some inner step failed, causing this error.
#[derive(Debug, Clone)]
pub struct SequentialMatchError<'a, R: pest::RuleType> {
    pub pair: pest::iterators::Pair<'a, R>,
    pub context: Rc<ParsingContext>,
    pub cause: Box<TreeError<'a, R>>
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
                .with_message(format!(
                    "Failed to match due member error"
                ))
            )
            .with_code(4)
            .with_message("Did not match Sequentially")
            .finish()
    }
    fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        let err = self.to_displayed_error();
        let mut disp: DisplayedTrace<'a> = DisplayedTrace { reports: vec![err] };
        let mut other_disp: DisplayedTrace<'a> = (*self.cause).clone().into();
        disp.add_cause(&mut other_disp);
        return disp;
    }
}