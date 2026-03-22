use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

// --- The Derive Macro ---
#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        // This implements the trait we define in main.rs
        impl Describe for #name {
            fn describe(&self) {
                println!("I am a struct named {}", stringify!(#name));
            }
        }
    };
    TokenStream::from(expanded)
}

// --- The Attribute Macro ---
#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;
    let block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    let expanded = quote! {
        #vis #sig {
            println!("--- Entering: {} ---", stringify!(#name));
            let result = { #block };
            println!("--- Exiting: {} ---", stringify!(#name));
            result
        }
    };
    TokenStream::from(expanded)
}