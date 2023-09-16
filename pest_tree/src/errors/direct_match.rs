//! Failed to match an exact string.
use super::super::*;
use super::*;
use ariadne::Fmt;
/// Refer to [`TreeError::DirectMatchError`].
#[derive(Debug, Clone)]
pub struct DirectMatchError<'a, R: pest::RuleType> {
    /// Pair that failed to match the rule.
    pub pair: pest::iterators::Pair<'a, R>,
    /// Refer to [`ParsingContext`].
    pub context: Rc<ParsingContext>,
    /// The identifier of the struct/enum/field that failed to match a rule.
    pub object_ident: String,
    /// The name of the rule. (e.g. `Rule::abc`)
    pub goal_rule: String,
}

impl<'a, R: pest::RuleType> TreeErrorVariant<'a, R> for DirectMatchError<'_, R> {
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        let range = self.get_range();
        ariadne::Report::build(ariadne::ReportKind::Error, self.context.filename.clone(), 0)
            .with_label(
                ariadne::Label::new((self.context.filename.clone(), range))
                    .with_message(format!(
                        "Could not match rule {} when trying to match {}. Rule given was {} ",
                        self.goal_rule
                            .to_string()
                            .as_str()
                            .fg(ariadne::Color::Magenta),
                        self.object_ident
                            .to_string()
                            .as_str()
                            .fg(ariadne::Color::Magenta),
                        format!("{:?}", self.get_rule())
                            .as_str()
                            .fg(ariadne::Color::Magenta)
                    ))
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

impl<'a, T: pest::RuleType> DirectMatchError<'a, T> {
    /// Construct a [`DirectMatchError`] as a [`TreeError`].
    pub fn as_tree_error(
        failing_pair: pest::iterators::Pair<'a, T>,
        context: Rc<ParsingContext>,
        ident: String,
        rule: String,
    ) -> TreeError<'_, T> {
        let err = Self {
            pair: failing_pair.clone(),
            context,
            object_ident: ident,
            goal_rule: rule,
        };
        TreeError::DirectMatchError(err)
    }
    fn get_rule(&self) -> T {
        self.pair.as_rule()
    }
}
