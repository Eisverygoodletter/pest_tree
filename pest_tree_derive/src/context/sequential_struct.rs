use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::attributes::*;
use crate::DeriveContext;
use crate::*;
use derive_builder::Builder;
use quote::format_ident;

#[derive(Builder, Clone, Debug, PartialEq)]
pub(crate) struct SequentialFieldContext {
    member_ident: syn::Ident,
    ty: syn::Type,
    require_attrs: Vec<RequireAttribute>,
    conversion_attr: ConvertAttribute,
}

impl StructFieldContext for SequentialFieldContext {
    fn from_syn_field(field: &syn::Field) -> Self {
        let basic_attributes = BasicAttribute::from_syn_attributes(&field.attrs);
        SequentialFieldContextBuilder::default()
            .member_ident(field.ident.clone().unwrap())
            .ty(field.ty.clone())
            .conversion_attr(
                basic_attributes
                    .iter()
                    .find_map(|attr| {
                        if let BasicAttribute::Convert(c) = attr {
                            Some(c)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(&ConvertAttribute::Auto)
                    .clone(),
            )
            .require_attrs(
                basic_attributes
                    .into_iter()
                    .filter_map(|attr| {
                        if let BasicAttribute::Require(req) = attr {
                            Some(req)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
            .build()
            .expect("failed to build sequential field context")
    }
}

impl SequentialFieldContext {
    // pub fn condition(&self) -> TokenStream {}
    pub fn check(&self, ctx: &DeriveContext) -> TokenStream {
        let field_ident = format_ident!("{}_{}_check_pair", &ctx.ident, self.member_ident);
        let checks = self
            .require_attrs
            .iter()
            .map(|req| req.check(&ctx, (&quote! {#field_ident}).into()));
        quote! {
            let #field_ident = inner.next().expect("impossible: Pairs length was checked");
            #(#checks)*
        }
    }
    pub fn field_pair(&self, ctx: &DeriveContext) -> TokenStream {
        let field_ident = format_ident!("{}_{}_check_pair", &ctx.ident, self.member_ident);
        let ident = &self.member_ident;
        let expr = self
            .conversion_attr
            .expression(&self.ty, Some(&quote! { #field_ident }));
        quote! {
            #ident: #expr,
        }
    }
}

/// Rules for strategy::Sequential are:
/// 1. Any number of overall requirements on the pair
/// 2. Any number of fields. The overall pair should contain a Pairs, which contains many [`pest::Pair`].
/// Each [`pest::Pair`] matches to one field.
/// Every [`pest::Pair`] is "consumed" when that field is converted
///
#[derive(Builder, Debug, PartialEq)]
pub(crate) struct SequentialStructContext {
    ctx: DeriveContext,
    fields: Vec<SequentialFieldContext>,
    require_attrs: Vec<RequireAttribute>,
}
impl StructContext for SequentialStructContext {
    fn from_syn_item_struct(item_struct: syn::ItemStruct) -> Self {
        let basic_attributes = BasicAttribute::from_syn_attributes(&item_struct.attrs);
        let derive_context = DeriveContextBuilder::default()
            .ident(item_struct.ident)
            .struct_enum_token(Ident::new("struct", proc_macro2::Span::call_site()))
            .rule_ident(
                BasicAttribute::search_for_rule_in_attrs(&basic_attributes)
                    .expect("A Rule requirement must be supplied for sequential matches"),
            )
            .build()
            .unwrap();
        SequentialStructContextBuilder::default()
            .ctx(derive_context)
            .fields(
                item_struct
                    .fields
                    .iter()
                    .map(SequentialFieldContext::from_syn_field)
                    .collect(),
            )
            .require_attrs(
                basic_attributes
                    .into_iter()
                    .filter_map(|attr| {
                        if let BasicAttribute::Require(req) = attr {
                            Some(req)
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
            .build()
            .unwrap()
    }
    fn to_impl(&self) -> TokenStream {
        let ident = &self.ctx.ident;
        let rule_ident = &self.ctx.rule_ident.segments[0].ident;
        let overall_require_checks: Vec<_> = self
            .require_attrs
            .iter()
            .map(|req| req.check(&self.ctx, None))
            .collect();
        let fields = &self.fields;
        let expected_count = fields.len();
        let field_checks = self.fields.iter().map(|f| f.check(&self.ctx));
        let field_pairs = self.fields.iter().map(|f| f.field_pair(&self.ctx));
        // panic!("{}", field_checks.clone().next().unwrap().to_string());
        quote! {
            #[allow(non_snake_case)]
            impl PestTree<#rule_ident> for #ident {
                fn from_pest(
                    pair: pest::iterators::Pair<'_, #rule_ident>,
                    context: std::rc::Rc<ParsingContext>,
                ) -> Result<Self, TreeError<#rule_ident>>
                where
                    Self: Sized,
                {
                    // checks for the overall pair
                    let check_pair = pair.clone();
                    #(#overall_require_checks)*
                    // expand the members
                    let mut inner = pair.clone().into_inner();
                    // check if there's a correct number of Pair in the Pairs
                    let inner_cloned_for_length = inner.clone();
                    let count_found = inner_cloned_for_length.count();
                    if #expected_count != count_found {
                        return Err(PairCountError::as_tree_error(
                            pair,
                            context,
                            #expected_count,
                            count_found
                        ))
                    }
                    #(#field_checks)*
                    // TODO: finish code for expanding the "inner" Pairs (go write the code for the wrong pair )
                    Ok(#ident {
                        #(#field_pairs)*
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derive_sequential_struct() {
        let item_struct: syn::ItemStruct = parse_quote! {
            #[derive(PestTree)]
            #[pest_tree(strategy(Sequential))]
            #[pest_tree(require(rule(Rule::abc)))]
            struct ABC {
                a: A,
                b: B,
                c: C
            }
        };
        let implementation = SequentialStructContext::from_syn_item_struct(item_struct);
        // panic!("{}", implementation.to_impl().to_string());
        panic!("{}", pretty_print(&implementation.to_impl()));
    }
}
