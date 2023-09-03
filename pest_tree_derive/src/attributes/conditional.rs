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

impl RequireAttribute {
    pub fn get_condition(&self) -> TokenStream {
        match self {
            Self::Rule(p) => quote!({check_pair.as_rule() == #p}),
            Self::Any(reqs) => {
                // cond || cond2 || cond3;
                let condition_assignments = reqs
                    .into_iter()
                    .map(|req| req.get_condition())
                    .fold(quote!(false), |acc, elem| quote!(#acc || #elem));
                quote!({#condition_assignments})
            }
            _ => unimplemented!("h"),
        }
    }
    pub fn get_fail_error(&self, ctx: &DeriveContext) -> TokenStream {
        match self {
            Self::Rule(p) => {
                let ident = &ctx.ident_with_type;
                quote!(Err(pest_tree::DirectMatchError::as_tree_error(
                    check_pair.clone(),
                    context.clone(),
                    stringify!(#ident).to_string(),
                    stringify!(#p).to_string()
                )))
            }
            Self::Any(reqs) => {
                quote!(panic!("forgot to implement"))
            }
            _ => unimplemented!("wow"),
        }
    }
    pub fn fail_if_not_condition(&self, ctx: &DeriveContext) -> TokenStream {
        let condition = self.get_condition();
        let failing_error = self.get_fail_error(ctx);
        quote!(
            let succeeded = #condition;
            if !succeeded {
                return #failing_error;
            }
        )
    }
}
