//! Failed to match an exact string.
use super::super::*;
use super::*;
use ariadne::Fmt;
/// Refer to [`TreeError::DirectMatchError`].
#[derive(Debug, Clone)]
pub struct ConditionalEnumError<'a, R: pest::RuleType> {
    /// Pair that failed to match the rule.
    pub pair: pest::iterators::Pair<'a, R>,
    /// Refer to [`ParsingContext`].
    pub context: Rc<ParsingContext>,
}

impl<'a, R: pest::RuleType> TreeErrorVariant<'a, R> for ConditionalEnumError<'_, R> {
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        let range = self.get_range();
        ariadne::Report::build(ariadne::ReportKind::Error, self.context.filename.clone(), 0)
            .with_label(
                ariadne::Label::new((self.context.filename.clone(), range))
                    .with_message(format!("Pair did not match any variants in enum"))
                    .with_color(ariadne::Color::Red),
            )
            .with_code(1)
            .with_message("Did not match rule")
            .finish()
    }
    fn get_context(&self) -> Rc<ParsingContext> {
        self.context.clone()
    }
    fn get_pair(&self) -> &pest::iterators::Pair<'_, R> {
        &self.pair
    }
}

impl<'a, T: pest::RuleType> ConditionalEnumError<'a, T> {
    /// Construct a [`ConditionalEnumError`] as a [`TreeError`].
    pub fn as_tree_error(
        failing_pair: pest::iterators::Pair<'a, T>,
        context: Rc<ParsingContext>,
        ident: String,
        rule: String,
    ) -> TreeError<'_, T> {
        let err = Self {
            pair: failing_pair.clone(),
            context,
        };
        TreeError::ConditionalEnumError(err)
    }
    fn get_rule(&self) -> T {
        self.pair.as_rule()
    }
}
