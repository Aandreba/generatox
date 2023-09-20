use proc_macro2::Span;
use quote::format_ident;
use std::collections::BTreeSet;
use syn::{fold::Fold, parse_quote, token::Await, Expr, ExprAwait, ExprYield, Lifetime, Type, GenericArgument};

#[derive(Default)]
pub struct DefineLifetimes {
    pub lts: BTreeSet<Lifetime>,
    pub unique: Option<Lifetime>,
}

impl DefineLifetimes {
    fn unique(&mut self) -> &Lifetime {
        if self.unique.is_none() {
            let lt = Lifetime {
                apostrophe: Span::call_site(),
                ident: format_ident!("__0"),
            };

            self.lts.insert(lt.clone());
            self.unique = Some(lt.clone());
        }

        return unsafe { self.unique.as_ref().unwrap_unchecked() }
    }
}

impl Fold for DefineLifetimes {
    fn fold_generics(&mut self, mut i: syn::Generics) -> syn::Generics {
        for lt in i.lifetimes_mut() {
            let unique = self.unique();

            if lt.lifetime.ident == "_" {
                lt.lifetime = unique.clone();
            } else {
                lt.bounds.push(unique.clone());
                lt.colon_token = Some(std::default::Default::default());
                self.lts.insert(lt.lifetime.clone());
            }

        }

        return syn::fold::fold_generics(self, i);
    }

    fn fold_generic_argument(&mut self, mut i: GenericArgument) -> GenericArgument {
        match &mut i {
            GenericArgument::Lifetime(i) if i.ident == "_" => {
                *i = self.unique().clone()
            },
            _ => {}
        }

        return syn::fold::fold_generic_argument(self, i);
    }

    fn fold_type_reference(&mut self, mut i: syn::TypeReference) -> syn::TypeReference {
        let unique = self.unique();

        match &mut i.lifetime {
            Some(lt) if lt.ident != "_" => {
                self.lts.insert(lt.clone());
            }
            _ => i.lifetime = Some(unique.clone()),
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
                    dot_token: std::default::Default::default(),
                    await_token: Await {
                        span: yield_token.span,
                    },
                });
            }
            other => syn::fold::fold_expr(self, other),
        }
    }
}

// Saluton patrino, cu estas bona "si" venas Ana hodiau?