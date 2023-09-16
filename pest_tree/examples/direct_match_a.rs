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

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct DirectParser;

#[derive(PestTree, Debug, PartialEq)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::a)))]
struct A {}

fn main() {
    // successful match
    let test_str = "a";
    let parsed: iterators::Pair<'_, Rule> = DirectParser::parse(Rule::a, test_str)
        .unwrap()
        .next()
        .unwrap();
    let ctx = pest_tree::ParsingContext {
        filename: "testfile.file".to_string(),
        contents: test_str.to_string(),
    };
    let a = A::with_pair(parsed, Rc::new(ctx)).unwrap();
    assert_eq!(a, A {});
    // unsuccessful match (rule b instead of a)
    let test_str = "b";
    let parsed = DirectParser::parse(Rule::b, test_str)
        .unwrap()
        .next()
        .unwrap();
    let ctx = pest_tree::ParsingContext {
        filename: "wrongfile.bad".to_string(),
        contents: test_str.to_string(),
    };
    let tree_error = A::with_pair(parsed, Rc::new(ctx)).unwrap_err();
    // pretty print error
    tree_error.eprint();
}
