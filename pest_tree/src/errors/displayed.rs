//! Contains [`DisplayedTrace`] and [`DisplayedError`] used for pretty printing errors.
use super::super::*;
/// A single error. Normally [`DisplayedTrace`]s should be used directly instead.
///
/// This type only contains 1 error, without the cause. For some [`TreeError`] variants, errors can lead to
/// more errors. [`DisplayedTrace`] is used because it contains many [`DisplayedError`]s.
#[derive(Debug)]
pub struct DisplayedError<'a> {
    /// The error report printer from [`ariadne`].
    pub report: ariadne::Report<'a, (String, std::ops::Range<usize>)>,
    /// The parsing context, including the filename and contents referred to by the report.
    pub context: Rc<ParsingContext>,
}
impl DisplayedError<'_> {
    /// Print the error to the console.
    pub fn eprint(&self) {
        self.report
            .eprint((
                self.context.filename.clone(),
                ariadne::Source::from(self.context.contents.as_str()),
            ))
            .unwrap();
    }
}

/// A set of related errors.
#[derive(Debug)]
pub struct DisplayedTrace<'a> {
    /// Set of errors.
    pub reports: Vec<DisplayedError<'a>>,
}
impl<'a> DisplayedTrace<'a> {
    /// Print the error to the console.
    pub fn eprint(&self) {
        for report in &self.reports {
            report.eprint();
        }
    }
    /// Add a cause for the previous errors. This function is used for internal purposes during the construction
    /// of the [`DisplayedTrace`].
    pub fn add_cause(&mut self, cause: &mut DisplayedTrace<'a>) {
        self.reports.append(&mut cause.reports);
    }
}
