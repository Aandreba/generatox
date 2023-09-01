use crate::transform::Transformer;
use parse::YieldFn;
use quote::quote;
use syn::{fold::Fold, parse_macro_input, parse_quote};

mod parse;
mod transform;

#[proc_macro]
pub fn generator(body: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let YieldFn {
        attrs,
        vis,
        mut sig,
        yield_ty: ty,
        block,
        ..
    } = parse_macro_input!(body as YieldFn);

    let block = Transformer(&ty).fold_block(block);
    let (arrow, output) = match sig.output {
        syn::ReturnType::Default => (Default::default(), parse_quote!(())),
        syn::ReturnType::Type(arrow, output) => (arrow, output),
    };

    sig.output = parse_quote! {
        #arrow impl ::generatox::Generator<Yield = #ty, Return = #output>
    };

    return quote! {
        #(#attrs)*
        #vis #sig {
            ::generatox::wrapper::Wrapper {
                fut: async move #block,
                cell: ::generatox::corelib::option::Option::None,
            }
        }
    }
    .into();
}
