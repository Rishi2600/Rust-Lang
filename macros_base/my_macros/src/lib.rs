use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input};

#[proc_macro_attribute]
pub fn time_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;
    let vis = &input.vis;

    let expanded = quote! {
        #vis #sig {
            let __start = ::std::time::Instant::now();
            let __result = { #block };
            println!("{} took {:?}", stringify!(#name), __start.elapsed());
            __result
        }
    };
    TokenStream::from(expanded)
}