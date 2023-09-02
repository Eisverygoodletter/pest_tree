#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
extern crate pest;
use std::rc::Rc;

use pest::*;
extern crate pest_derive;
#[macro_use]
extern crate pest_tree_derive;
extern crate pest_tree;
use pest_derive::*;
use pest_tree::PestTree;
use pest_tree::TreeError;
use pest_tree::*;
use pest_tree_derive::PestTree;

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct DirectParser;

fn main() {
    let test_str = "adf";
    let res = DirectParser::parse(Rule::a, test_str).unwrap();
    let conversion_error = DirectMatchError::as_tree_error(
        res.into_iter().next().unwrap(),
        Rc::new(ParsingContext {
            filename: "pretty printed errors".to_string(),
            contents: test_str.to_string(),
        }),
    );
    conversion_error.eprint();
}
