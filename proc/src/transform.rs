use syn::fold::Fold;

pub struct CreateFuture {}

impl Fold for CreateFuture {
    fn fold_expr_yield(&mut self, i: syn::ExprYield) -> syn::ExprYield {}
}
