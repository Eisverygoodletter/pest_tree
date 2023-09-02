use crate::*;

pub trait TreeErrorVariant<'a, R: pest::RuleType> {
    fn get_context(&self) -> Rc<ParsingContext>;
    fn get_pair(&self) -> &pest::iterators::Pair<'_, R>;
    fn to_report(&self) -> ariadne::Report<'a, (String, std::ops::Range<usize>)>;
    /// DO NOT IMPLEMENT
    fn get_range(&self) -> std::ops::Range<usize> {
        let mut tokens = self.get_pair().clone().tokens();
        let pest::Token::Start { rule, pos } = tokens.next().expect("invalid iterator") else {
            panic!("invalid token")
        };
        let start_pos = pos.pos();
        let pest::Token::End { rule, pos } = tokens.next().expect("invalid iterator") else {
            panic!("invalid token")
        };
        let end_pos = pos.pos();
        start_pos..end_pos
    }
    /// DO NOT IMPLEMENT
    fn to_displayed_error(&self) -> DisplayedError<'a> {
        let rep = self.to_report();
        DisplayedError {
            report: rep,
            context: self.get_context(),
        }
    }
    /// DO NOT IMPLEMENT
    fn to_displayed_trace(&self) -> DisplayedTrace<'a> {
        let err = self.to_displayed_error();
        DisplayedTrace { reports: vec![err] }
    }
}
