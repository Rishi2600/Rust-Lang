use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn inject_db(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. NEW: Parse the attribute argument (the URL string)
    let db_url = parse_macro_input!(attr as LitStr);

    // 2. Parse the function
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;

    // 3. Reconstruct with the dynamic URL
    let expanded = quote! {
        #fn_vis fn #fn_name() {
            // Internal Helper Struct
            struct Database {
                url: String,
            }
            impl Database {
                fn query(&self, sql: &str) {
                    println!("--- DI LOGGER ---");
                    println!("Target: {}", self.url);
                    println!("SQL:    {}", sql);
                }
            }

            // Injected using the macro argument!
            let db = Database { 
                url: #db_url.into() 
            };

            // Execute original code
            let original_logic = || {
                #fn_block
            };
            
            original_logic();
        }
    };

    TokenStream::from(expanded)
}