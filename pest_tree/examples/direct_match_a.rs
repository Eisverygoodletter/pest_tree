#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
//! This module demonstrates the Direct match strategy
//! The rules are:
//! ```pest
//! a = { "a" }
//! ```
extern crate pest;
use pest::*;
extern crate pest_derive;
#[macro_use]
extern crate pest_tree_derive;
extern crate pest_tree;
use pest_derive::*;
use pest_tree::PestTree;
use pest_tree::TreeError;
use pest_tree_derive::PestTree;

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct DirectParser;

#[derive(PestTree, Debug)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::a)))]
struct A {}

fn main() {
    // let test_str = "abc";
    // let parsed = DirectParser::parse(Rule::a, test_str);
    // let res = A::from_pest(parsed.unwrap());
    // let tree_error = res.unwrap_err();
    // tree_error.print_report(test_str);
    // println!("the result is {:#?}", tree_error.generate_report(test_str));
}
