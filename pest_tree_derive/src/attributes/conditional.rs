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
    /// Get the rule name the attribute is referring to.
    /// If a variant was "Rule::a", the rule name would be `Rule`
    pub fn rule_enum_name(&self) -> Option<syn::Path> {
        match &self {
            Self::Any(v) => {
                for requirement in v {
                    let res = requirement.rule_enum_name();
                    if res.is_some() {
                        return res;
                    }
                }
                None
            }
            Self::Rule(p) => Some(p.clone()),
            _ => None,
        }
    }
    /// Get the condition as an expression
    pub fn condition(&self, check_pair: Option<&TokenStream>) -> TokenStream {
        let v = quote!(check_pair);
        let check_pair = check_pair.unwrap_or(&v);
        /**
         * TODO:
         * Fix not creating an expression
         */
        match self {
            Self::Rule(p) => quote! {#check_pair.as_rule() == #p},
            Self::Any(requirements) => {
                let conditions = requirements
                    .iter()
                    .map(|requirement| requirement.condition(Some(check_pair)))
                    .fold(quote!(false), |acc, elem| quote!(#acc || #elem));
                quote!(#conditions)
            }
            _ => unimplemented!("validation requirements are unimplemented"),
        }
    }
    pub fn error(&self, ctx: &DeriveContext, check_pair: Option<&TokenStream>) -> TokenStream {
        let v = quote!(check_pair);
        let check_pair = check_pair.unwrap_or(&v);
        let ident = &ctx.ident;
        match self {
            Self::Rule(p) => {
                let ident = &ctx.ident;
                quote!(Err(pest_tree::DirectMatchError::as_tree_error(
                    #check_pair.clone(),
                    context.clone(),
                    stringify!(#ident).to_string(),
                    stringify!(#p).to_string()
                )))
            }
            Self::Any(reqs) => {
                quote!(panic!("any error hasn't been implemented"))
            }
            _ => unimplemented!("wow"),
        }
    }
    /// [`RequireAttribute::condition`] and [`RequireAttribute::error`] combined
    pub fn check(&self, ctx: &DeriveContext, check_pair: Option<&TokenStream>) -> TokenStream {
        let expr = self.condition(check_pair);
        let err = self.error(ctx, check_pair);
        quote! {
            if !(#expr) {
                return #err;
            }
        }
    }
}

// impl RequireAttribute {
//     pub fn get_fail_error(&self, ctx: &DeriveContext) -> TokenStream {
//         match self {
//             Self::Rule(p) => {
//                 let ident = &ctx.ident_with_type;
//                 quote!(Err(pest_tree::DirectMatchError::as_tree_error(
//                     check_pair.clone(),
//                     context.clone(),
//                     stringify!(#ident).to_string(),
//                     stringify!(#p).to_string()
//                 )))
//             }
//             Self::Any(reqs) => {
//                 quote!(panic!("forgot to implement"))
//             }
//             _ => unimplemented!("wow"),
//         }
//     }
//     pub fn fail_if_not_condition(&self, ctx: &DeriveContext) -> TokenStream {
//         let condition = self.get_condition();
//         let failing_error = self.get_fail_error(ctx);
//         quote!(
//             let succeeded = #condition;
//             if !succeeded {
//                 return #failing_error;
//             }
//         )
//     }
// }
