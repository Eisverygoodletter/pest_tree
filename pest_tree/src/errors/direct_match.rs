use super::super::*;
use super::*;
use ariadne::Fmt;
/// Failed to match an exact string.
#[derive(Debug, Clone)]
pub struct DirectMatchError<'a, R: pest::RuleType> {
    pub pair: pest::iterators::Pair<'a, R>,
    pub context: Rc<ParsingContext>,
    pub object_ident: String,
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
                        format!("{}", self.goal_rule)
                            .as_str()
                            .fg(ariadne::Color::Magenta),
                        format!("{}", self.object_ident)
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
