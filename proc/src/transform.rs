use syn::{fold::Fold, parse_quote, token::Await, Expr, ExprAwait, ExprYield, Type};

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
                    base: parse_quote! { ::generatox::wrapper::r#yield::<#ty>(#expr) },
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
