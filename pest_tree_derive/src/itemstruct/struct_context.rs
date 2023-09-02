use super::super::*;
use super::*;
#[derive(Debug)]
pub(crate) struct FieldDeriveContext {
    ident: syn::Ident,
    requirements: Vec<RequireAttribute>,
    converter: Option<ConvertAttribute>,
}
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
            BasicAttribute::Require(req) => match req {
                RequireAttribute::Rule(r) => self.rule = r,
                _ => self.overall_requirements.push(req),
            },
            BasicAttribute::Strategy(strat) => self.overall_strategy = strat,
        }
    }
    pub fn get_implementation_token_stream(&self) -> TokenStream {
        let ident = &self.ident;

        let rule = &self.rule.segments[0].ident;
        quote!(
            impl PestTree<#rule> for #ident {
                fn from_pest(
                    pairs: pest::iterators::Pair<'_, Self::PestRule>,
                    context: Rc<ParsingContext>
                ) -> Result<Self, TreeError<Self::PestRule>> {

                }
            }
        )
    }
}
