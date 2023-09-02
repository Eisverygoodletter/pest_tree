#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, Meta, Token, Variant,
};

pub(crate) mod struct_context;
pub use struct_context::*;
fn pretty_print(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
}
use crate::attributes::BasicAttribute;
pub(crate) fn struct_derive(item: syn::ItemStruct) -> TokenStream {
    let mut ctx = StructDeriveContext::build(item.ident.clone());
    for attr in &item.attrs {
        if !attr.path().is_ident("pest_tree") {
            // might be for another macro unrelated to pest_tree
            continue;
        }
        let parsed_basic = attr.parse_args_with(BasicAttribute::parse);
        if let Ok(attr) = parsed_basic {
            ctx.add_overall_attr(attr);
        }
    }
    for field in &item.fields {
        for attr in &field.attrs {
            if !attr.path().is_ident("pest_tree") {
                continue;
            }
            let parsed_basic = attr.parse_args_with(BasicAttribute::parse);
            if let Ok(attr) = parsed_basic {
                // add it todo
            } else {
                panic!("failed to parse field attribute {:#?}", attr);
            }
        }
    }
    let s1 = pretty_print((&item.clone().into_token_stream()).into());
    let s2 = pretty_print(&ctx.get_implementation_token_stream());
    panic!(
        "\n{}------------------------------------------------------------\n{}",
        s1, s2
    );
    ctx.get_implementation_token_stream().into()
}
