#![allow(
    dead_code,
    irrefutable_let_patterns,
    unused_variables,
    unused_imports,
    unused
)]
use crate::attributes::*;
use derive_builder::Builder;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    braced, parse_macro_input, parse_quote, token, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Expr, Fields, GenericParam, Generics, Index, ItemEnum, ItemStruct, Meta, Token,
    Variant,
};
/// Includes information about the struct/enum that the macro is deriving for.
#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub(crate) struct DeriveContext {
    /// Identifier (e.g. the "A" struct `A`)
    pub ident: syn::Ident,
    /// Either `struct` or `enum`.
    pub struct_enum_token: syn::Ident,
    /// Normally this is "Rule"
    pub rule_ident: syn::Path,
}

pub(crate) trait StructContext {
    fn from_syn_item_struct(item_struct: ItemStruct) -> Self;
    fn to_impl(&self) -> TokenStream;
}

pub(crate) trait EnumContext {
    fn from_syn_item_enum(item_enum: ItemEnum) -> Self;
    fn to_impl(&self) -> TokenStream;
}

pub(crate) trait StructFieldContext {
    fn from_syn_field(field: &syn::Field) -> Self;
}

pub(crate) mod direct_struct;
pub(crate) use direct_struct::*;
pub(crate) mod sequential_struct;
pub(crate) use sequential_struct::*;
