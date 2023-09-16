//! Failed to automatically convert from a [`str`] to the specified type.
use super::*;
use ariadne::Fmt;

/// Refer to [`TreeError::StringConversionError`].
#[derive(Debug, Clone)]
pub struct StringConversionError<'a, R: pest::RuleType> {
    /// The pair whose [`str`] couldn't be converted into the specified type.
    pub pair: pest::iterators::Pair<'a, R>,
    /// Refer to [`ParsingContext`]
    pub context: Rc<ParsingContext>,
    /// The name of the type that [`str::parse`] was trying to convert to.
    pub goal_type_name: String,
}

impl<'a, R: pest::RuleType> TreeErrorVariant<'a, R> for StringConversionError<'_, R> {
    fn get_context(&self) -> Rc<ParsingContext> {
        self.context.clone()
    }
    fn get_pair(&self) -> &pest::iterators::Pair<'_, R> {
        &self.pair
    }
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)> {
        let range = self.get_range();
        ariadne::Report::build(ariadne::ReportKind::Error, self.context.filename.clone(), 0)
            .with_label(
                ariadne::Label::new((self.context.filename.clone(), range))
                    .with_message(format!(
                        "Failed to convert Pair to {}",
                        format!("{:?}", self.goal_type_name)
                            .as_str()
                            .fg(ariadne::Color::Magenta)
                    ))
                    .with_color(ariadne::Color::Red),
            )
            .with_code(2)
            .with_message(format!(
                "{} failed to convert {} to {}",
                "#[pest_tree(convert(auto))]".fg(ariadne::Color::Blue),
                "Pair".fg(ariadne::Color::Blue),
                self.goal_type_name.as_str().fg(ariadne::Color::Blue),
            ))
            .finish()
    }
}

impl<'a, R: pest::RuleType> StringConversionError<'a, R> {
    /// Create a [`StringConversionError`] wrapped in a [`TreeError`] by referring to the 
    /// error created by [`str::parse`].
    pub fn from_str_conversion_error<T: std::str::FromStr>(
        pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
    ) -> TreeError<'_, R> {
        TreeError::StringConversionError(Self {
            pair,
            context,
            goal_type_name: std::any::type_name::<T>().to_string(),
        })
    }
}
