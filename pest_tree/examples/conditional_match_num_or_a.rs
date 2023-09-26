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
use pest_tree::*;
use pest_tree_derive::PestTree;

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct DirectParser;
#[derive(PestTree, Debug, PartialEq)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::a)))]
pub struct A {}

#[derive(PestTree, Debug, PartialEq)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::num)))]
struct Num {
    pub number: i32,
}

#[derive(PestTree, Debug, PartialEq)]
#[pest_tree(strategy(Conditional))]
#[pest_tree(require(rule(Rule::num_or_a)))]
enum NumOrA {
    Num(Num),
    A(A),
}

fn main() {
    // successful match
    let test_str = "32";
    let parsed: iterators::Pair<'_, Rule> = DirectParser::parse(Rule::num_or_a, test_str)
        .unwrap()
        .next()
        .unwrap();
    let ctx = pest_tree::ParsingContext {
        filename: "testfile.file".to_string(),
        contents: test_str.to_string(),
    };
    let num_32 = NumOrA::with_pair(parsed, Rc::new(ctx)).unwrap();
    assert_eq!(num_32, NumOrA::Num(Num { number: 32 }));
}
