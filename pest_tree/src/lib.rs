#![allow(dead_code, irrefutable_let_patterns, unused_variables, unused_imports, unused)]
pub use ariadne;
use core::hash::Hash;
use ariadne::{Color, ColorGenerator, Fmt, Label, Report, ReportKind, Source, ReportBuilder};
#[derive(Debug)]
pub enum ConversionError {
    NoMatch(Label),
}

impl ConversionError {
    pub fn get_report_label(&self) -> Label {
        match self {
            Self::NoMatch(value) => value.clone()
        }
    }
}
use pest::{iterators::Pair, RuleType};
use syn::token;
#[derive(Debug)]
pub struct TreeError<'a, T: RuleType + Ord> {
    pub err: ConversionError,
    pub cause: Option<Box<TreeError<'a, T>>>,
    pub location: Pair<'a, T>,
}
impl<T: RuleType> TreeError<'_, T> {
    pub fn attach_report_label(&self, report: &mut ReportBuilder<core::ops::Range<usize>>) {
        let local_err = self.err.get_report_label();
        let mut token_iter = self.location.clone().tokens();
        let start_token = token_iter.next().unwrap();
        let end_token = token_iter.next().unwrap();
        let pest::Token::Start { rule, pos } = start_token else { panic!("incorrect start token") };
        let start_pos = pos.pos();
        let pest::Token::End { rule, pos } = end_token else { panic!("incorrect start token") };
        let end_pos = pos.pos();
        
        report.add_label(
            self.err.get_report_label()
        );
        if let Some(boxed_error) = &self.cause {
            boxed_error.as_ref().attach_report_label(report);
        }
    }
    pub fn generate_report(&self, source: &str) -> Report<'_> {
        let mut report: ReportBuilder<core::ops::Range<usize>> = Report::build(ReportKind::Error, (), 12);
        self.attach_report_label(&mut report);
        report.finish()
    }
    pub fn print_report(&self, source: &str) {
        let report = self.generate_report(source);
        let _ = report.print(Source::from(source));
    }
}

/**
 * C
 */
pub trait PestTree {
    type PestRule;
    fn from_pest(
        pairs: pest::iterators::Pairs<'_, Self::PestRule>,
    ) -> Result<Self, TreeError<Self::PestRule>>
    where
        Self: Sized,
        <Self as PestTree>::PestRule: RuleType;
}
