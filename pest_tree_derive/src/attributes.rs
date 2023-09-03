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
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};

pub mod conditional;
pub use conditional::*;
pub mod converter;
pub use converter::*;

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
    }
    // rule
    pub mod requirement {
        syn::custom_keyword!(rule);
        syn::custom_keyword!(validate);
        syn::custom_keyword!(any);
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

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum StrategyAttribute {
    Direct,
    Sequential,
}
impl Parse for StrategyAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::strategy::Direct) {
            return Ok(Self::Direct);
        }
        if input.peek(kw::strategy::Sequential) {
            return Ok(Self::Sequential);
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
    Validate,
    Any(Vec<RequireAttribute>),
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
        unimplemented!("other require attributes");
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ConvertAttribute {
    CustomP,
    CustomS,
    Auto,
}
impl Parse for ConvertAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::convert::auto) {
            return Ok(Self::Auto);
        } else if input.peek(kw::convert::custom_p) {
            let _ = input.parse::<kw::convert::custom_p>();
            let inner;
            syn::parenthesized!(inner in input);
            panic!("opos");
            // todo
        }
        panic!("convert fail {:#?}", input);
    }
}
