#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
use std::fmt::Debug;

use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};

mod itemstruct;
use itemstruct::struct_derive;
mod strategy;
use strategy::*;
mod attributes;
use attributes::*;

#[proc_macro_derive(PestTree, attributes(pest_tree))]
pub fn pest_tree_derive(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: syn::Item = parse_macro_input!(token_stream);
    match ast {
        syn::Item::Struct(item_struct) => struct_derive(item_struct).into(),
        _ => todo!(),
    }
}
