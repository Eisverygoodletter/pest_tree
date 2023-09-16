extern crate pest;
use pest_tree::ParsingContext;
use std::rc::Rc;

use pest::*;
extern crate pest_derive;
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
#[pest_tree(require(rule(Rule::num)))]
struct Num {
    pub number: i32
}

fn main() {
    // successful match
    let test_str = "32";
    let parsed: iterators::Pair<'_, Rule> = DirectParser::parse(Rule::num, test_str)
        .unwrap()
        .next()
        .unwrap();
    let ctx = pest_tree::ParsingContext {
        filename: "testfile.file".to_string(),
        contents: test_str.to_string(),
    };
    let num = Num::with_pair(parsed, Rc::new(ctx)).unwrap();
    println!("the number is {}", num.number);
    assert_eq!(num.number, 32);
}
