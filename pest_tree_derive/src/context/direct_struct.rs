use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::attributes::*;
use crate::DeriveContext;
use crate::*;
use derive_builder::Builder;
#[derive(Builder, Clone, Debug, PartialEq)]
pub(crate) struct DirectFieldContext {
    member_ident: syn::Ident,
    ty: syn::Type,
    conversion_attr: ConvertAttribute,
}
impl StructFieldContext for DirectFieldContext {
    fn from_syn_field(field: &syn::Field) -> Self {
        let basic_attributes = BasicAttribute::from_syn_attributes(&field.attrs);
        DirectFieldContextBuilder::default()
            .member_ident(
                field
                    .ident
                    .as_ref()
                    .expect("missing field identifier")
                    .clone(),
            )
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
            .ty(field.ty.clone())
            .build()
            .unwrap()
    }
}

impl ToTokens for DirectFieldContext {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.member_ident;
        let expression = &self.conversion_attr.expression(&self.ty, None);
        tokens.extend(quote! {
            #ident: #expression,
        });
    }
}

/// Rules for strategy::Direct are:
/// 1. Any number of overall requirements on the pair
/// 2. Any number of fields. All fields are directly converted from the pair.
#[derive(Builder, Debug, PartialEq)]
pub(crate) struct DirectStructContext {
    ctx: DeriveContext,
    fields: Vec<DirectFieldContext>,
    require_attrs: Vec<RequireAttribute>,
}

impl StructContext for DirectStructContext {
    fn from_syn_item_struct(item_struct: syn::ItemStruct) -> Self {
        let basic_attributes = BasicAttribute::from_syn_attributes(&item_struct.attrs);
        let derive_context = DeriveContextBuilder::default()
            .ident(item_struct.ident)
            .struct_enum_token(Ident::new("struct", proc_macro2::Span::call_site()))
            .rule_ident(
                BasicAttribute::search_for_rule_in_attrs(&basic_attributes)
                    .expect("A Rule requirement must be supplied for direct matches"),
            )
            .build()
            .unwrap();
        DirectStructContextBuilder::default()
            .ctx(derive_context)
            .fields(
                item_struct
                    .fields
                    .iter()
                    .map(DirectFieldContext::from_syn_field)
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
    fn to_impl(&self) -> proc_macro2::TokenStream {
        let ident = &self.ctx.ident;
        let rule_ident = &self.ctx.rule_ident.segments[0].ident;
        let require_checks: Vec<_> = self
            .require_attrs
            .iter()
            .map(|req| req.check(&self.ctx, None))
            .collect();
        let fields = &self.fields;
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
                    let check_pair = pair.clone();
                    #(#require_checks)*
                    let convert_pair = pair.clone();
                    Ok(#ident {
                        #(#fields)*
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
    fn derive_direct_struct() {
        let item_struct: syn::ItemStruct = parse_quote! {
            #[derive(PestTree)]
            #[pest_tree(strategy(Direct))]
            #[pest_tree(require(rule(Rule::a)))]
            struct A {
                #[pest_tree(convert(auto))]
                converted: String
            }
        };
        let implementation = DirectStructContext::from_syn_item_struct(item_struct);
        let expected = quote! {
            impl PestTree<Rule> for A {
                fn from_pest(
                    pair: pest::iterators::Pair<'_, Rule>,
                    context: Rc<ParsingContext>,
                ) -> Result<Self, TreeError<Rule>>
                where
                    Self: Sized,
                {
                    let check_pair = pair.clone();
                    if !(check_pair.as_rule() == Rule::a) {
                        return Err(
                            pest_tree::DirectMatchError::as_tree_error(
                                check_pair.clone(),
                                context.clone(),
                                stringify!(A).to_string(),
                                stringify!(Rule::a).to_string(),
                            ),
                        );
                    }
                    let convert_pair = pair.clone();
                    Ok(A {
                        converted: String::from_pest(convert_pair),
                    })
                }
            }
        };
        // implementation.to_impl().eq
        // panic!("{}", pretty_print(&implementation.to_impl()));
        use syn::ItemImpl;
        let expected: ItemImpl = syn::parse2(expected).unwrap();
        let generated: ItemImpl = syn::parse2(implementation.to_impl()).unwrap();
        assert_eq!(
            pretty_print(&expected.to_token_stream()),
            pretty_print(&generated.to_token_stream())
        );
    }
}
