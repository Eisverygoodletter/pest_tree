#![allow(dead_code, irrefutable_let_patterns, unused_variables, unused_imports, unused)]
use std::fmt::Debug;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput,
    Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};

#[derive(Debug, PartialEq)]
enum Strategy {
    Direct,
    Sequential,
    Undecided,
}
enum Requirement {
    MatchesRule(syn::Path),
}
impl Requirement {
    fn from_expr(expr: Expr) -> Self {
        let Expr::Call(expr) = expr else {
            panic!("expected a further call")
        };
        let Expr::Path(p) = expr.func.as_ref() else {
            panic!("wow {:#?}", expr)
        };
        if p.path.is_ident("rule") {
            // its a `MatchesRule` requirement
            if expr.args.len() != 1 {
                panic!("you can only match 1 rule within a `rule(<rule>)` expression");
            }
            let Expr::Path(ref p) = expr.args[0] else {
                panic!("INTERNAL ERROR");
            };
            return Self::MatchesRule(p.path.clone());
        }
        panic!("Failed to parse requirement (internal error)");
    }
    fn to_condition(&self, content: TokenStream) -> TokenStream {
        match self {
            Self::MatchesRule(p) => {
                quote!(
                    if pair.as_rule() != #p {
                        #content
                    }
                )
            }
        }
    }
    fn condition_check(&self) -> TokenStream {
        let content = match self {
            Self::MatchesRule(p) => {
                quote!(
                    return Err(TreeError::<Self::PestRule> {
                        err: pest_tree::ariadne::Label::new(),
                        cause: Option::None,
                        location: pair
                    })
                )
            }
        };
        self.to_condition(content)
    }
}

struct ConversionStrategy {
    pub strategy: Strategy,
    pub requirements: Vec<Requirement>,
    pub rule_ident: Ident,
}
impl From<Vec<Attribute>> for ConversionStrategy {
    fn from(value: Vec<Attribute>) -> Self {
        let mut strategy = Strategy::Undecided;
        let mut requirements: Vec<Requirement> = vec![];
        for attr in value {
            if attr.style != syn::AttrStyle::Outer {
                panic!("inner attribute styles not supported");
            }
            let Meta::List(l) = attr.meta else {
                panic!("unsupported attribute");
            };
            if l.path.segments[0].ident != "pest_tree" {
                continue;
            }
            // finding out whether it is "strategy" or "require"
            //panic!("{:#?}", l.tokens);
            let expr: syn::ExprCall = l
                .parse_args()
                .expect("invalid syntax: Should be in the form of option(options)");
            let Expr::Path(p) = expr.func.as_ref() else {
                panic!("Expected path (internal error)");
            };
            let path = &p.path;
            if path.is_ident("strategy") {
                // check for potential errors
                if strategy != Strategy::Undecided {
                    panic!("Cannot have multiple strategies for 1 struct/enum. Strategy first chosen was {:?}", strategy);
                }
                if expr.args.len() != 1 {
                    panic!(
                        "expected 1 argument in strategy(...). Found {}",
                        expr.args.len()
                    );
                }
                // extract the option from the strategy(option)
                let Expr::Path(p) = &expr.args[0] else {
                    panic!("Must be either strategy(ident) or require(...)");
                };
                let ident = p
                    .path
                    .get_ident()
                    .expect("identifier within strategy(ident) is missing.");
                let ident = ident.to_string();
                strategy = Strategy::from(ident.as_str());
                continue;
            }
            if path.is_ident("require") {
                if expr.args.is_empty() {
                    panic!("cannot have empty require(...)");
                }
                for arg in expr.args.into_iter() {
                    let req: Requirement = Requirement::from_expr(arg);

                    requirements.push(req);
                }
                continue;
            }
            panic!("unrecognised token; should be strategy(ident) or require(...)");
        }
        let rule_ident = requirements
            .iter()
            .find_map(|requirement| {
                if let Requirement::MatchesRule(r) = requirement {
                    Some(r.segments[0].ident.clone())
                } else {
                    None
                }
            })
            .expect("could not find Rule");
        Self {
            strategy,
            requirements,
            rule_ident: rule_ident,
        }
    }
}
impl From<&str> for Strategy {
    fn from(value: &str) -> Self {
        match value {
            "Direct" => Self::Direct,
            "Sequential" => Self::Sequential,
            _ => panic!("invalid strategy"),
        }
    }
}

#[proc_macro_derive(PestTree, attributes(pest_tree))]
pub fn derive_pest_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    if !ast.generics.params.is_empty() {
        panic!("pest structs/enums involving generics are not supported.");
    }
    match ast.data {
        Data::Enum(_) => derive_data_enum(ast),
        Data::Struct(_) => derive_data_struct(ast),
        Data::Union(_) => panic!("Unions are not supported."),
    }
}

fn derive_data_enum(ast: DeriveInput) -> proc_macro::TokenStream {
    let strategy = ConversionStrategy::from(ast.attrs);

    TokenStream::new().into()
}
fn derive_data_struct(ast: DeriveInput) -> proc_macro::TokenStream {
    let conversion_strategy = ConversionStrategy::from(ast.attrs);
    let ident = ast.ident;
    let rule_ident = conversion_strategy.rule_ident;
    let r = conversion_strategy.requirements[0].condition_check();
    quote!(
        impl PestTree for #ident {
            type PestRule = #rule_ident;
            fn from_pest(pairs: pest::iterators::Pairs<'_, Self::PestRule>) -> Result<Self, TreeError<Self::PestRule>> {
                let pair = pairs.peek().unwrap();
                #r
                todo!();
            }
        }
    )
    .into()
}
