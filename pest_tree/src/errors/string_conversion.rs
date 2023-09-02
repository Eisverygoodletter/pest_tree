use super::*;
use ariadne::Fmt;
#[derive(Debug, Clone)]
pub struct StringConversionError<'a, R: pest::RuleType> {
    pub pair: pest::iterators::Pair<'a, R>,
    pub context: Rc<ParsingContext>,
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
