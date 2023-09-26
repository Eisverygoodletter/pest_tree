#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
extern crate pest;
use pest_tree::ParsingContext;
use std::rc::Rc;

use pest::*;
extern crate pest_derive;
#[macro_use]
extern crate pest_tree_derive;
extern crate pest_tree;
use pest_derive::*;
use pest_tree::PestTree;
use pest_tree::TreeError;
use pest_tree_derive::PestTree;
use pest_tree::*;

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct DirectParser;

#[derive(PestTree, Debug, PartialEq)]
#[pest_tree(strategy(Conditional))]
#[pest_tree(require(rule(Rule::boolean)))]
enum Boolean {
    #[pest_tree(require(matches("true")))]
    True,
    #[pest_tree(require(matches("false")))]
    False,
}

fn main() {
    // successful match
    let test_str = "true";
    let parsed: iterators::Pair<'_, Rule> = DirectParser::parse(Rule::boolean, test_str)
        .unwrap()
        .next()
        .unwrap();
    let ctx = pest_tree::ParsingContext {
        filename: "testfile.file".to_string(),
        contents: test_str.to_string(),
    };
    let a = Boolean::with_pair(parsed, Rc::new(ctx)).unwrap();
    assert_eq!(a, Boolean::True);
}
