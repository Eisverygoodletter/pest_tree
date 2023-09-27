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
use pest_tree::ParsingContext;
use pest_tree::PestTree;
use pest_tree::TreeError;
use pest_tree::*;
use pest_tree_derive::PestTree;

#[derive(Parser)]
#[grammar = "../examples/sequential.pest"]
pub struct SequentialParser;

#[derive(PestTree, Debug, Clone)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::d)))]
pub struct D {}
#[derive(PestTree, Debug, Clone)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::e)))]
pub struct E {}
#[derive(PestTree, Debug, Clone)]
#[pest_tree(strategy(Direct))]
#[pest_tree(require(rule(Rule::f)))]
pub struct F {}
#[derive(PestTree, Debug, Clone)]
#[pest_tree(strategy(Sequential))]
#[pest_tree(require(rule(Rule::def)))]
pub struct DEF {
    #[pest_tree(skippable)]
    pub d: Option<D>,
    #[pest_tree(skippable)]
    pub e: Option<E>,
    #[pest_tree(skippable)]
    pub f: Option<F>,
}

fn main() {
    let test_str = "d";
    let ctx = pest_tree::ParsingContext {
        filename: "testfile.file".to_string(),
        contents: test_str.to_string(),
    };
    let parsed = SequentialParser::parse(Rule::def, test_str).unwrap();
    let def = DEF::with_pair(parsed.into_iter().next().unwrap(), std::rc::Rc::new(ctx));
    // let a = def.unwrap_err();
    // a.eprint();
    println!("{:#?}", def.unwrap());

    // let bad_test_str = "abc";
    // let res = Seq::from_pest(parsed.unwrap());
    // let tree_error = res.unwrap_err();
    // tree_error.print_report(test_str);
    // println!("the result is {:#?}", tree_error.generate_report(test_str));
}
