use crate::transform::Transformer;
use parse::YieldFn;
use quote::quote;
use syn::{fold::Fold, parse_macro_input, parse_quote, GenericParam, LifetimeParam};
use transform::DefineLifetimes;

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

    let mut lts = DefineLifetimes::default();
    sig = lts.fold_signature(sig);
    sig.generics.params.extend(
        lts.unique
            .iter()
            .cloned()
            .map(|x| GenericParam::Lifetime(LifetimeParam::new(x))),
    );

    // let lts = lts.lts.iter().fold(TokenStream::new(), |prev, x| quote!(#prev #x +));
    let lts = lts.unique.map(|x| quote!(#x +));

    let block = Transformer(&ty).fold_block(block);
    let (arrow, output) = match sig.output {
        syn::ReturnType::Default => (Default::default(), parse_quote!(())),
        syn::ReturnType::Type(arrow, output) => (arrow, output),
    };

    sig.output = parse_quote! {
        #arrow impl #lts ::generatox::Generator<Yield = #ty, Return = #output>
    };

    return quote! {
        #(#attrs)*
        #vis #sig {
            ::generatox::wrapper::Wrapper::new(async move #block)
        }
    }
    .into();
}
