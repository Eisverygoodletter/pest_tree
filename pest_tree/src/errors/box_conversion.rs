use super::*;
use ariadne::Fmt;
/// Failing to contain a type T within a Box.
#[derive(Debug, Clone)]
pub struct BoxConversionError<'a, R: pest::RuleType> {
    pub pair: pest::iterators::Pair<'a, R>,
    pub context: Rc<ParsingContext>,
    pub inner_type_name: String,
}
impl<'a, R: pest::RuleType> TreeErrorVariant<'a, R> for BoxConversionError<'_, R> {
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
                        "Failed to contain Pair within Box<{}>",
                        format!("{:?}", self.inner_type_name)
                            .as_str()
                            .fg(ariadne::Color::Magenta)
                    ))
                    .with_color(ariadne::Color::Red),
            )
            .with_code(2)
            .with_message(format!(
                "{} failed to store {} within Box<{}>",
                "#[pest_tree(convert(auto))]".fg(ariadne::Color::Blue),
                "Pair".fg(ariadne::Color::Blue),
                self.inner_type_name.as_str().fg(ariadne::Color::Blue),
            ))
            .finish()
    }
}

impl<'a, R: pest::RuleType> BoxConversionError<'a, R> {
    pub fn from_type<T>(
        pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
    ) -> TreeError<'_, R> {
        TreeError::BoxConversionError(Self {
            pair,
            context,
            inner_type_name: std::any::type_name::<T>().to_string(),
        })
    }
}
