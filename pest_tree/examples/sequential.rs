#![allow(unused)]
//! This module demonstrates the sequential conversion strategy.
//! The rules are:
//! ```pest
//! a = { "a" }
//! b = { "b" }
//! c = { "c" }
//! seq = { a ~ b ~ c }
//! ```
//! Where struct `Seq` contains the members `a`, `b` and `c`.
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
pub struct SequentialParser;

#[derive(PestTree, Debug)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::a)))]
pub struct A {}
#[derive(PestTree, Debug)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::b)))]
pub struct B {}
#[derive(PestTree, Debug)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::c)))]
pub struct C {}
#[derive(PestTree, Debug)]
#[pest_tree(strategy(Sequential))]
#[pest_tree(require(rule(Rule::seq)))]
pub struct Seq {
    pub a: A,
    pub b: B,
    pub c: C,
}

fn main() {
    let test_str = "abc";
    let parsed = SequentialParser::parse(Rule::a, test_str);
    // let res = Seq::from_pest(parsed.unwrap());
    // let tree_error = res.unwrap_err();
    // tree_error.print_report(test_str);
    // println!("the result is {:#?}", tree_error.generate_report(test_str));
}
