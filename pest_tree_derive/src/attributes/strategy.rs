use std::fmt::Debug;

use super::*;
use super::*;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};

impl StrategyAttribute {
    pub fn from_syn_attributes(attrs: &[Attribute]) -> Self {
        let attrs = BasicAttribute::from_syn_attributes(attrs);
        attrs
            .iter()
            .find_map(|attr| {
                if let BasicAttribute::Strategy(strat) = attr {
                    Some(strat)
                } else {
                    None
                }
            })
            .expect("missing strategy")
            .clone()
    }
}
