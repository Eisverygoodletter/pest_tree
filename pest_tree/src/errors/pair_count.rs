/// Error for not having the correct number of pair in a Pairs
use super::super::*;
use super::*;
use ariadne::Fmt;
#[derive(Debug, Clone)]
pub struct PairCountError<'a, R: pest::RuleType> {
    pub pair: pest::iterators::Pair<'a, R>,
    pub context: Rc<ParsingContext>,
    pub expected_count: usize,
    pub count_found: usize,
}

impl<'a, R: pest::RuleType + 'a> TreeErrorVariant<'a, R> for PairCountError<'a, R> {
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
                    "Expected {} 'Pair's, but got {} in Pairs",
                    self.expected_count.to_string().fg(ariadne::Color::Magenta),
                    self.count_found.to_string().fg(ariadne::Color::Magenta)
                )),
            )
            .with_code(4)
            .with_message("Wrong number of Pair")
            .finish()
    }
}

impl<'a, R: pest::RuleType> PairCountError<'a, R> {
    pub fn with_count(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        expected_count: usize,
        count_found: usize,
    ) -> Self {
        Self {
            pair: failing_pair,
            context,
            expected_count,
            count_found,
        }
    }
    pub fn as_tree_error(
        failing_pair: pest::iterators::Pair<'a, R>,
        context: Rc<ParsingContext>,
        expected_count: usize,
        count_found: usize,
    ) -> TreeError<'_, R> {
        TreeError::PairCountError(Self::with_count(
            failing_pair,
            context,
            expected_count,
            count_found,
        ))
    }
}
