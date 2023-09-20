use proc_macro2::Span;
use quote::format_ident;
use std::collections::BTreeSet;
use syn::{fold::Fold, parse_quote, token::Await, Expr, ExprAwait, ExprYield, Lifetime, Type};

#[derive(Default)]
pub struct DefineLifetimes {
    pub lts: BTreeSet<Lifetime>,
    pub uniques: Vec<Lifetime>,
    pub last_idx: usize,
}

impl Fold for DefineLifetimes {
    fn fold_generics(&mut self, i: syn::Generics) -> syn::Generics {
        for lt in i.lifetimes() {
            self.lts.insert(lt.lifetime.clone());
        }
        return syn::fold::fold_generics(self, i);
    }

    fn fold_type_reference(&mut self, mut i: syn::TypeReference) -> syn::TypeReference {
        match &i.lifetime {
            Some(lt) => {
                self.lts.insert(lt.clone());
            }
            None => {
                let mut lt;
                loop {
                    lt = Lifetime {
                        apostrophe: Span::call_site(),
                        ident: format_ident!("__{}", self.last_idx),
                    };

                    self.last_idx += 1;
                    if self.lts.insert(lt.clone()) {
                        break;
                    }
                }

                self.uniques.push(lt.clone());
                i.lifetime = Some(lt);
            }
        }

        return syn::fold::fold_type_reference(self, i);
    }
}

pub struct Transformer<'a>(pub &'a Type);

impl Fold for Transformer<'_> {
    fn fold_expr(&mut self, i: syn::Expr) -> syn::Expr {
        match i {
            Expr::Yield(ExprYield {
                attrs,
                yield_token,
                expr,
            }) => {
                let expr = expr.unwrap_or_else(|| parse_quote! { () });
                let ty = self.0;

                return Expr::Await(ExprAwait {
                    attrs,
                    base: parse_quote! { unsafe { ::generatox::wrapper::r#yield::<#ty>(#expr) } },
                    dot_token: Default::default(),
                    await_token: Await {
                        span: yield_token.span,
                    },
                });
            }
            other => syn::fold::fold_expr(self, other),
        }
    }
}
