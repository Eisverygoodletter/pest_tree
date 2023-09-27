//! A pair did not match a string.
use super::super::*;
use super::*;
use ariadne::Fmt;
/// Refer to [`TreeError::WrongHierarchyError`].
#[derive(Debug, Clone)]
pub struct WrongHierarchyError<'a, R: pest::RuleType> {
    /// Pair which contained the `Pairs` with wrong number of fields.
    pub pair: pest::iterators::Pair<'a, R>,
    /// Refer to [`ParsingContext`].
    pub context: Rc<ParsingContext>,
    /// The name of the type that [`str::parse`] was trying to convert to.
    pub goal_type_name: String,
}

impl<'a, R: pest::RuleType + 'a> TreeErrorVariant<'a, R> for WrongHierarchyError<'a, R> {
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
                    "The Pair {} does not have the correct hierarchical structure for {}", 
                    self.pair.as_str().to_string().fg(ariadne::Color::Magenta),
                    self.goal_type_name.clone().fg(ariadne::Color::Magenta)
                )),
            )
            .with_code(4)
            .with_message("Did not match String")
            .finish()
    }
}

impl<'a, R: pest::RuleType> WrongHierarchyError<'a, R> {
    /// Initialize a [`WrongHierarchyError`].
    pub fn with_name(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        goal_type_name: String,
    ) -> Self {
        Self {
            pair: failing_pair,
            context,
            goal_type_name,
        }
    }
    /// Initialize a [`WrongHierarchyError`] wrapped within a [`TreeError`].
    pub fn as_tree_error(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        goal_type_name: String,
    ) -> TreeError<'a, R> {
        TreeError::WrongHierarchyError(Self::with_name(failing_pair, context, goal_type_name))
    }
}
