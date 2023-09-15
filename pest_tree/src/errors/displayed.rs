use super::super::*;
#[derive(Debug)]
pub struct DisplayedError<'a> {
    pub report: ariadne::Report<'a, (String, std::ops::Range<usize>)>,
    pub context: Rc<ParsingContext>,
}
impl DisplayedError<'_> {
    pub fn eprint(&self) {
        self.report
            .eprint((
                self.context.filename.clone(),
                ariadne::Source::from(self.context.contents.as_str()),
            ))
            .unwrap();
    }
}

#[derive(Debug)]
pub struct DisplayedTrace<'a> {
    pub reports: Vec<DisplayedError<'a>>,
}
impl<'a> DisplayedTrace<'a> {
    pub fn eprint(&self) {
        for report in &self.reports {
            report.eprint();
        }
    }
    pub fn add_cause(&mut self, cause: &mut DisplayedTrace<'a>) {
        self.reports.append(&mut cause.reports);
    }
}
