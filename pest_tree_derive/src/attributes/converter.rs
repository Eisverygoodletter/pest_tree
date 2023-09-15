#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
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

impl ConvertAttribute {
    pub fn expression(
        &self,
        end_type: &syn::Type,
        convert_pair: Option<&TokenStream>,
    ) -> TokenStream {
        let v = quote!(convert_pair);
        let convert_pair = convert_pair.unwrap_or(&v);
        match self {
            Self::Auto => {
                quote! {
                    #end_type::from_pest(#convert_pair, context.clone())?
                }
            }
            Self::CustomP(pair_converter) => {
                quote! {
                    #pair_converter(#convert_pair)
                }
            }
            Self::CustomS(string_converter) => {
                quote! {
                    #string_converter(<#convert_pair>.as_str())
                }
            } // todo: chain(...) expressions
        }
    }
}
