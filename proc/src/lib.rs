use proc_macro2::TokenStream;
use syn::{parse_macro_input, Stmt};

mod transform;

#[proc_macro_attribute]
pub fn generator(
    attrs: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let f = parse_macro_input!(body as syn::ItemFn);
}
