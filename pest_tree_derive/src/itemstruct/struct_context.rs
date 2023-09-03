use crate::attributes::kw::basic::require;

use super::super::*;
use super::*;
#[derive(Debug, Clone)]
pub(crate) struct FieldDeriveContext {
    ident: syn::Ident,
    requirements: Vec<RequireAttribute>,
    converters: Vec<ConvertAttribute>,
}
fn get_success_token_stream(requirements: &Vec<RequireAttribute>) -> TokenStream {
    requirements
        .into_iter()
        .map(|req| req.get_condition())
        .fold(quote!(false), |acc, elem| quote!(#acc || #elem))
}
impl FieldDeriveContext {
    pub fn build(ident: syn::Ident) -> Self {
        Self {
            ident,
            requirements: vec![],
            converters: vec![],
        }
    }
    pub fn add_attr(&mut self, attr: BasicAttribute) {
        match attr {
            BasicAttribute::Strategy(_) => panic!("Strategy attributes cannot be used on fields."),
            BasicAttribute::Require(req) => self.requirements.push(req),
            BasicAttribute::Convert(conv) => self.converters.push(conv),
        }
    }
    pub fn get_content(&self) -> TokenStream {
        quote!()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct StructDeriveContext {
    ident: syn::Ident,
    rule: syn::Path,
    overall_strategy: StrategyAttribute,
    overall_requirements: Vec<RequireAttribute>,
    fields: Vec<FieldDeriveContext>,
}
impl StructDeriveContext {
    pub fn build(ident: syn::Ident) -> Self {
        Self {
            ident,
            rule: syn::Path::from(syn::Ident::new(
                "InternalError",
                proc_macro2::Span::call_site(),
            )),
            overall_strategy: StrategyAttribute::Direct,
            overall_requirements: vec![],
            fields: vec![],
        }
    }
    pub fn add_overall_attr(&mut self, attr: BasicAttribute) {
        match attr {
            BasicAttribute::Convert(_) => {
                panic!("converter attributes are not allowed for non-members")
            }
            BasicAttribute::Require(req) => match &req {
                RequireAttribute::Rule(r) => {
                    self.rule = r.clone();
                    self.overall_requirements.push(req);
                }
                RequireAttribute::Any(v) => {
                    if self.rule
                        == syn::Path::from(syn::Ident::new(
                            "InternalError",
                            proc_macro2::Span::call_site(),
                        ))
                    {
                        // search for rule() within the any()
                        for req in v {
                            match req {
                                RequireAttribute::Rule(r) => self.rule = r.clone(),
                                _ => {}
                            }
                        }
                    }
                    self.overall_requirements.push(req);
                }
                _ => self.overall_requirements.push(req),
            },
            BasicAttribute::Strategy(strat) => self.overall_strategy = strat,
        }
    }
    pub fn get_if_checks(&self, ctx: &DeriveContext) -> Vec<TokenStream> {
        let mut require_token_streams: Vec<TokenStream> = vec![];
        for req in &self.overall_requirements {
            require_token_streams.push(req.fail_if_not_condition(ctx));
        }
        require_token_streams
    }
    pub fn get_content(&self) -> Vec<TokenStream> {
        if self.overall_strategy == StrategyAttribute::Direct {
            let ident = &self.ident;
            return vec![quote!(
                return Ok(#ident {});
            )];
        } else if self.overall_strategy == StrategyAttribute::Sequential {
            return self
                .fields
                .clone()
                .into_iter()
                .map(|field| field.get_content())
                .collect::<Vec<_>>();
        }
        panic!("unimplemented");
    }
    pub fn get_implementation_token_stream(&self) -> TokenStream {
        let ctx = DeriveContext {
            ident: self.ident.clone(),
            ident_with_type: "struct ".to_string() + self.ident.to_string().as_str(),
        };
        let ident = &self.ident;
        let rule = &self.rule.segments[0].ident;
        let if_checks = self.get_if_checks(&ctx);
        let content = self.get_content();
        quote!(
            impl PestTree<#rule> for #ident {
                fn from_pest(
                    pair: ::pest::iterators::Pair<'_, #rule>,
                    context: std::rc::Rc<pest_tree::ParsingContext>
                ) -> Result<Self, TreeError<#rule>> {
                    let check_pair = pair.clone();
                    #(#if_checks)*
                    #(#content)*
                }
            }
        )
    }
}
