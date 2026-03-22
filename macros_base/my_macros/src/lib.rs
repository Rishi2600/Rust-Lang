use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    // 1. Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // 2. Generate the implementation
    let expanded = quote! {
        impl Describe for #name {
            fn describe(&self) {
                println!("I am a struct named {}", stringify!(#name));
            }
        }
    };

    // 3. Convert back to TokenStream
    TokenStream::from(expanded)
}