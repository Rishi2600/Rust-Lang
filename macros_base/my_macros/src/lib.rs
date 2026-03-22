use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the function we're wrapping
    let input = parse_macro_input!(item as ItemFn);
    
    // 2. Extract parts of the function
    let name = &input.sig.ident;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    // 3. Generate the "Wrapper"
    // We recreate the function signature and put our logic inside
    let expanded = quote! {
        #vis #sig {
            println!(">> Entering: {}", stringify!(#name));
            
            // Execute the original block of code
            let result = { #block };
            
            println!("<< Exiting: {}", stringify!(#name));
            
            // Return the result of the block
            result
        }
    };

    TokenStream::from(expanded)
}