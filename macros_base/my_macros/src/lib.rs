use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, FnArg, Pat};

#[proc_macro_attribute]
pub fn get_page(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the Attribute (DI: Database URL)
    let db_url = parse_macro_input!(attr as LitStr);

    // 2. Parse the Function (CLI: Argument Introspection)
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    
    // Get the name of the first argument (e.g., "user_id")
    let arg_name = if let Some(FnArg::Typed(pat_type)) = input_fn.sig.inputs.first() {
        if let Pat::Ident(ref id) = *pat_type.pat {
            id.ident.to_string()
        } else { "id".into() }
    } else { "id".into() };

    // 3. Reconstruct (The Grand Summary)
    let expanded = quote! {
        fn #fn_name(raw_input: i32) -> String {
            // Internal Logic (Project 4: DI)
            let db_connection = #db_url;
            let #fn_name = #arg_name; // Mapping input to user's variable name
            
            println!("--- LOG: Fetching {} from {} ---", #arg_name, db_connection);

            // Execute the user's logic (Project 1 & 2: DSL/Mapper)
            let body: String = (|| #fn_block)();

            // Project 1: Wrapping in a final HTML template
            format!("<html><body>{}</body></html>", body)
        }
    };

    TokenStream::from(expanded)
}