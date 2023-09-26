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
                fn with_pair(
                    pair: pest::iterators::Pair<'_, #rule_ident>,
                    context: std::rc::Rc<ParsingContext>,
                ) -> Result<Self, TreeError<#rule_ident>>
                where
                    Self: Sized,
                {
                    // checks for the overall pair
                    let check_pair = pair.clone();
                    let backup_pair = pair.clone();
                    let backup_ctx = context.clone();
                    let conversion_result = move || -> Result<Self, pest_tree::TreeError<'_, #rule_ident>> {
                        #(#overall_require_checks)*
                        // expand the members
                        let mut inner = pair.clone().into_inner();
                        // check if there's a correct number of Pair in the Pairs
                        let inner_cloned_for_length = inner.clone();
                        let count_found = inner_cloned_for_length.count();
                        if #expected_count != count_found {
                            return Err(PairCountError::as_tree_error(
                                check_pair,
                                context,
                                #expected_count,
                                count_found
                            ))
                        }
                        #(#field_checks)*
                        // TODO: finish code for expanding the "inner" Pairs (go write the code for the wrong pair )
                        return Ok(#ident {
                            #(#field_pairs)*
                        });
                    }();
                    conversion_result.map_err(|err| {
                        SequentialMatchError::as_tree_error(
                            backup_pair,
                            backup_ctx,
                            err
                        )
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
        let expected = quote! {
            #[allow(non_snake_case)]
            impl PestTree<Rule> for ABC {
                fn with_pair(
                    pair: pest::iterators::Pair<'_, Rule>,
                    context: std::rc::Rc<ParsingContext>,
                ) -> Result<Self, TreeError<Rule>>
                where
                    Self: Sized,
                {
                    let check_pair = pair.clone();
                    let backup_pair = pair.clone();
                    let backup_ctx = context.clone();
                    let conversion_result = move || -> Result<Self, pest_tree::TreeError<'_, Rule>> {
                        if !(check_pair.as_rule() == Rule::abc) {
                            return Err(
                                pest_tree::DirectMatchError::as_tree_error(
                                    check_pair.clone(),
                                    context.clone(),
                                    stringify!(ABC).to_string(),
                                    stringify!(Rule::abc).to_string(),
                                ),
                            );
                        }
                        let mut inner = pair.clone().into_inner();
                        let inner_cloned_for_length = inner.clone();
                        let count_found = inner_cloned_for_length.count();
                        if 3usize != count_found {
                            return Err(
                                PairCountError::as_tree_error(check_pair, context, 3usize, count_found),
                            );
                        }
                        let ABC_a_check_pair = inner
                            .next()
                            .expect("impossible: Pairs length was checked");
                        let ABC_b_check_pair = inner
                            .next()
                            .expect("impossible: Pairs length was checked");
                        let ABC_c_check_pair = inner
                            .next()
                            .expect("impossible: Pairs length was checked");
                        return Ok(ABC {
                            a: <A>::with_pair(ABC_a_check_pair, context.clone())?,
                            b: <B>::with_pair(ABC_b_check_pair, context.clone())?,
                            c: <C>::with_pair(ABC_c_check_pair, context.clone())?,
                        });
                    }();
                    conversion_result
                        .map_err(|err| {
                            SequentialMatchError::as_tree_error(backup_pair, backup_ctx, err)
                        })
                }
            }
        };
        use syn::ItemImpl;
        let expected: ItemImpl = syn::parse2(expected).unwrap();
        let generated: ItemImpl = syn::parse2(implementation.to_impl()).unwrap();
        assert_eq!(
            pretty_print(&expected.to_token_stream()),
            pretty_print(&generated.to_token_stream())
        );
    }
}
