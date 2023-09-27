use proc_macro2::TokenStream;
use quote::TokenStreamExt;

use crate::attributes::*;
use crate::DeriveContext;
use crate::*;
use derive_builder::Builder;
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ConditionalVariantContext {
    Unit {
        reqs: Vec<RequireAttribute>,
        ident: syn::Ident,
    },
    Unnamed {
        tys: Vec<syn::Type>,
        reqs: Vec<RequireAttribute>,
        ident: syn::Ident,
    },
}
impl ConditionalVariantContext {
    fn from_syn_fields(field: &syn::Fields, attrs: &[syn::Attribute], ident: &syn::Ident) -> Self {
        let reqs = BasicAttribute::from_syn_attributes(attrs)
            .into_iter()
            .filter_map(|f| {
                if let BasicAttribute::Require(v) = f {
                    return Some(v);
                }
                return None;
            })
            .collect();
        match field {
            syn::Fields::Unit => ConditionalVariantContext::Unit {
                reqs,
                ident: ident.clone(),
            },
            syn::Fields::Unnamed(unnamed) => ConditionalVariantContext::Unnamed {
                tys: unnamed.unnamed.clone().into_iter().map(|f| f.ty).collect(),
                reqs,
                ident: ident.clone(),
            },
            _ => unreachable!("cannot have named enum fields"),
        }
    }
    fn attempt_return(&self, ctx: &DeriveContext) -> TokenStream {
        match self {
            Self::Unit { reqs, ident } => {
                let conditions: Vec<_> = reqs.iter().map(|f| f.check(ctx, None)).collect();
                let rule_ident = &ctx.rule_ident.segments[0].ident;
                quote! {
                    let check_pair = pair.clone();
                    let context = backup_context.clone();
                    let success = move || -> Result<Self, pest_tree::TreeError<'_, #rule_ident>> {
                        #(#conditions)*
                        return Ok(Self::#ident);// todo
                    }();
                    if let Ok(s) = success {
                        return Ok(s);
                    }
                }
            }
            Self::Unnamed { tys, reqs, ident } => {
                let conditions: Vec<_> = reqs.iter().map(|f| f.check(ctx, None)).collect();
                let rule_ident = &ctx.rule_ident.segments[0].ident;
                let field_pairs = tys.iter().map(|f| {
                    quote! {
                        <#f>::with_pair(convert_pair, context.clone())?,
                    }
                });
                quote! {
                    let check_pair = pair.clone();
                    let context = backup_context.clone();
                    let convert_pair = backup_convert_pair.clone();
                    let success = move || -> Result<Self, pest_tree::TreeError<'_, #rule_ident>> {
                        #(#conditions)*
                        return Ok(Self::#ident(
                            #(#field_pairs)*
                        ));
                    }();
                    if let Ok(s) = success {
                        return Ok(s);
                    }
                }
            }
        }
    }
}
impl EnumVariantContext for ConditionalVariantContext {
    fn from_syn_variant(variant: &syn::Variant) -> Self {
        Self::from_syn_fields(&variant.fields, &variant.attrs, &variant.ident)
    }
}

#[derive(Builder, Debug, PartialEq)]
pub(crate) struct ConditionalEnumContext {
    ctx: DeriveContext,
    variants: Vec<ConditionalVariantContext>,
    require_attrs: Vec<RequireAttribute>,
}

impl EnumContext for ConditionalEnumContext {
    fn from_syn_item_enum(item_enum: syn::ItemEnum) -> Self {
        let basic_attributes = BasicAttribute::from_syn_attributes(&item_enum.attrs);
        let derive_context = DeriveContextBuilder::default()
            .ident(item_enum.ident)
            .struct_enum_token(Ident::new("enum", proc_macro2::Span::call_site()))
            .rule_ident(
                BasicAttribute::search_for_rule_in_attrs(&basic_attributes)
                    .expect("A Rule requirement must be supplied for sequential matches"),
            )
            .build()
            .unwrap();
        ConditionalEnumContextBuilder::default()
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
            .ctx(derive_context)
            .variants(
                item_enum
                    .variants
                    .iter()
                    .map(EnumVariantContext::from_syn_variant)
                    .collect(), //todo
            )
            .build()
            .unwrap()
    }
    fn to_impl(&self) -> TokenStream {
        let rule_ident = &self.ctx.rule_ident.segments[0].ident;
        // panic!("rule ident is {:#?}", rule_ident);
        let ident = &self.ctx.ident;
        let overall_require_checks: Vec<_> = self
            .require_attrs
            .iter()
            .map(|req| req.check(&self.ctx, None))
            .collect();
        let attempted_returns = self.variants.iter().map(|f| f.attempt_return(&self.ctx));
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
                    let backup_context = context.clone();
                    let mut convert_pairs = pair.clone().into_inner();
                    let potential_backup_convert_pair = convert_pairs.next();
                    if potential_backup_convert_pair.is_none() {
                        return Err(WrongHierarchyError::as_tree_error(
                            pair.clone(),
                            backup_context.clone(),
                            stringify!(#ident).to_string()
                        ))
                    }
                    let backup_convert_pair = potential_backup_convert_pair.unwrap();
                    #(#overall_require_checks)*
                    #(#attempted_returns)*
                    return Err(ConditionalEnumError::as_tree_error(
                        pair.clone(),
                        backup_context.clone(),
                        stringify!(#ident).to_string(),
                        stringify!(#rule_ident).to_string(),
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn derive_conditional_enum() {
        let item_enum: syn::ItemEnum = parse_quote! {
            #[derive(PestTree)]
            #[pest_tree(strategy(Conditional))]
            #[pest_tree(require(rule(Rule::abc)))]
            enum ABC {
                #[pest_tree(require(matches("a")))]
                A,
                #[pest_tree(require(matches("b")))]
                B,
                #[pest_tree(require(matches("c")))]
                C(Value)
            }
        };
        let implementation = ConditionalEnumContext::from_syn_item_enum(item_enum).to_impl();
        panic!("{}", pretty_print(&implementation));
    }
}
