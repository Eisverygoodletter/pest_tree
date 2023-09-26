#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
use std::fmt::Debug;

use crate::attributes::kw::basic::require;

use super::*;
use proc_macro2::{Ident, Punct, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, LitStr, Meta, Token, Variant,
};
pub mod conditional;
pub use conditional::*;
pub mod converter;
pub use converter::*;
pub mod strategy;
pub use strategy::*;

pub mod kw {
    // basic
    pub mod basic {
        syn::custom_keyword!(strategy);
        syn::custom_keyword!(require);
        syn::custom_keyword!(convert);
        syn::custom_keyword!(step);
    }
    // strategy
    pub mod strategy {
        syn::custom_keyword!(Direct);
        syn::custom_keyword!(Sequential);
        syn::custom_keyword!(Conditional);
    }
    // rule
    pub mod requirement {
        syn::custom_keyword!(rule);
        syn::custom_keyword!(validate);
        syn::custom_keyword!(any);
        syn::custom_keyword!(matches);
    }
    // convert
    pub mod convert {
        syn::custom_keyword!(custom_p);
        syn::custom_keyword!(custom_s);
        syn::custom_keyword!(auto);
    }
    pub mod step {
        syn::custom_keyword!(skip);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum BasicAttribute {
    Strategy(StrategyAttribute),
    Require(RequireAttribute),
    Convert(ConvertAttribute),
}
impl Parse for BasicAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // if input.peek(kw::Basic::strategy) { return Ok(Self::Strategy(input.parse()))}
        // panic!("{:#?}", input);

        if input.peek(kw::basic::strategy) {
            let _ = input.parse::<kw::basic::strategy>();
            // inner is necessary for extracting `(Direct)` -> `Direct`
            let inner;
            syn::parenthesized!(inner in input);
            let strat = inner.call(StrategyAttribute::parse)?;
            let _ = inner.parse::<TokenStream>();
            // panic!("alright {:#?}", strat);
            return Ok(BasicAttribute::Strategy(strat));
        }
        if input.peek(kw::basic::require) {
            let _ = input.parse::<kw::basic::require>();
            let inner;
            syn::parenthesized!(inner in input);
            let req = inner.call(RequireAttribute::parse)?;
            return Ok(BasicAttribute::Require(req));
        }
        if input.peek(kw::basic::convert) {
            let _ = input.parse::<kw::basic::convert>();
            let inner;
            syn::parenthesized!(inner in input);
            let converter = inner.call(ConvertAttribute::parse)?;
            return Ok(BasicAttribute::Convert(converter));
        }
        panic!("no strat {:#?}", input);
        Err(syn::Error::new(
            input.span(),
            "could not identify basic element",
        ))
    }
}

impl BasicAttribute {
    pub fn from_syn_attribute(attr: &Attribute) -> Option<Self> {
        if !attr.path().is_ident("pest_tree") {
            return None;
        }
        if let Ok(attr) = attr.parse_args_with(BasicAttribute::parse) {
            Some(attr)
        } else {
            panic!("failed to parse attribute");
        }
    }
    pub fn from_syn_attributes(attrs: &[Attribute]) -> Vec<Self> {
        attrs
            .iter()
            .filter_map(BasicAttribute::from_syn_attribute)
            .collect()
    }
    fn search_for_rule(&self) -> Option<syn::Path> {
        if let Self::Require(req) = self {
            req.rule_enum_name()
        } else {
            None
        }
    }
    pub fn search_for_rule_in_attrs(attrs: &[Self]) -> Option<syn::Path> {
        attrs
            .iter()
            .find(|v| v.search_for_rule().is_some())
            .and_then(|v| v.search_for_rule())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StrategyAttribute {
    Direct,
    Sequential,
    Conditional,
}
impl Parse for StrategyAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::strategy::Direct) {
            return Ok(Self::Direct);
        }
        if input.peek(kw::strategy::Sequential) {
            return Ok(Self::Sequential);
        }
        if input.peek(kw::strategy::Conditional) {
            return Ok(Self::Conditional);
        }
        Err(syn::Error::new(
            input.span(),
            "could not identify as Direct or Sequential",
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum RequireAttribute {
    Rule(syn::Path),
    Validate(syn::Expr),
    Any(Vec<RequireAttribute>),
    Matches(String),
}
impl Parse for RequireAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::requirement::rule) {
            let _ = input.parse::<kw::requirement::rule>();
            let inner;
            syn::parenthesized!(inner in input);
            let rule_path = syn::Path::parse(&inner)?;
            return Ok(Self::Rule(rule_path));
        }
        if input.peek(kw::requirement::any) {
            let _ = input.parse::<kw::requirement::any>();
            let inner;
            syn::parenthesized!(inner in input);
            let requirements = Punctuated::<RequireAttribute, Token!(,)>::parse_terminated(&inner)?;
            let iter = requirements.into_iter();
            let collected_vec = iter.collect::<Vec<_>>();
            return Ok(Self::Any(collected_vec));
        }
        if input.peek(kw::requirement::validate) {
            let _ = input.parse::<kw::requirement::validate>();
            let inner;
            syn::parenthesized!(inner in input);
            let callable_expression = syn::Expr::parse(&inner)?;
            return Ok(Self::Validate(callable_expression));
        }
        if input.peek(kw::requirement::matches) {
            let _ = input.parse::<kw::requirement::matches>();
            let inner;
            syn::parenthesized!(inner in input);
            let Ok(syn::Lit::Str(s)) = syn::Lit::parse(&inner) else {
                panic!("expected string literal in matches");
            };
            return Ok(Self::Matches(s.value()));
        }
        panic!("invalid require attribute");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ConvertAttribute {
    CustomP(syn::Expr),
    CustomS(syn::Expr),
    Auto,
}
impl Parse for ConvertAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::convert::auto) {
            let _ = input.parse::<TokenStream>();
            return Ok(Self::Auto);
        } else if input.peek(kw::convert::custom_p) {
            let _ = input.parse::<kw::convert::custom_p>();
            let inner;
            syn::parenthesized!(inner in input);
            let callable_expression = syn::Expr::parse(&inner)?;
            let _ = inner.parse::<TokenStream>();
            return Ok(Self::CustomP(callable_expression));
        } else if input.peek(kw::convert::custom_s) {
            let _ = input.parse::<kw::convert::custom_s>();
            let inner;
            syn::parenthesized!(inner in input);
            let callable_expression = syn::Expr::parse(&inner)?;
            let _ = inner.parse::<TokenStream>();
            return Ok(Self::CustomS(callable_expression));
        }
        panic!("invalid convert attribute");
    }
}
