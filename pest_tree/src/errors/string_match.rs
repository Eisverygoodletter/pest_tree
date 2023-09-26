//! A pair did not match a string.
use super::super::*;
use super::*;
use ariadne::Fmt;
/// Refer to [`TreeError::StringMatchError`].
#[derive(Debug, Clone)]
pub struct StringMatchError<'a, R: pest::RuleType> {
    /// Pair which contained the `Pairs` with wrong number of fields.
    pub pair: pest::iterators::Pair<'a, R>,
    /// Refer to [`ParsingContext`].
    pub context: Rc<ParsingContext>,
    /// What the pair should have matched.
    pub expected_string: String,
}

impl<'a, R: pest::RuleType + 'a> TreeErrorVariant<'a, R> for StringMatchError<'a, R> {
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
                ariadne::Label::new((self.context.filename.clone(), range)).with_message(format!(
                    "Expected Pair {} to be {}", // make this string match TODO
                    self.pair.as_str().to_string().fg(ariadne::Color::Magenta),
                    self.expected_string.clone().fg(ariadne::Color::Magenta)
                )),
            )
            .with_code(4)
            .with_message("Did not match String")
            .finish()
    }
}

impl<'a, R: pest::RuleType> StringMatchError<'a, R> {
    /// Initialize a [`StringMatchError`].
    pub fn with_string(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        expected_string: String,
    ) -> Self {
        Self {
            pair: failing_pair,
            context,
            expected_string,
        }
    }
    /// Initialize a [`StringMatchError`] wrapped within a [`TreeError`].
    pub fn as_tree_error(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        expected_string: String,
    ) -> TreeError<'a, R> {
        TreeError::StringMatchError(Self::with_string(failing_pair, context, expected_string))
    }
}
