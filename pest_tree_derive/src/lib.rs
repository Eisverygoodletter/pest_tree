#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
use derive_builder::Builder;
use std::fmt::Debug;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};
mod strategy;
use strategy::*;
mod attributes;
use attributes::*;
mod context;
use context::*;

#[proc_macro_derive(PestTree, attributes(pest_tree))]
pub fn pest_tree_derive(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::Item = parse_macro_input!(token_stream);
    match ast {
        syn::Item::Struct(item_struct) => {
            let strat = StrategyAttribute::from_syn_attributes(&item_struct.attrs);
            match strat {
                StrategyAttribute::Direct => {
                    let ctx = DirectStructContext::from_syn_item_struct(item_struct);
                    ctx.to_impl().into()
                }
                StrategyAttribute::Sequential => {
                    let ctx = SequentialStructContext::from_syn_item_struct(item_struct);
                    ctx.to_impl().into()
                }
            }
        }
        _ => todo!(),
    }
}

fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).expect("pretty print died");
    prettyplease::unparse(&file)
}
