use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn log_with_target(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    let target = attr.to_string().replace("\"", ""); // Clean up quotes
    
    let name = &input.sig.ident;
    let block = &input.block;
    let sig = &input.sig;

    let expanded = quote! {
        #sig {
            println!("[{}] Calling function: {}", #target, stringify!(#name));
            let result = { #block };
            result
        }
    };
    TokenStream::from(expanded)
}