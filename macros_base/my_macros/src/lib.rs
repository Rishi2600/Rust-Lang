use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn inject_db(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the function the user wrote
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    // We ignore the original arguments because we are "injecting" them ourselves
    
    // 2. Reconstruct the function
    // We define the 'Database' type and the 'db' variable INSIDE the new function
    let expanded = quote! {
        #fn_vis fn #fn_name() {
            // --- The Injection ---
            struct Database {
                connection_string: String,
            }
            impl Database {
                fn query(&self, sql: &str) {
                    println!("Executing '{}' on {}", sql, self.connection_string);
                }
            }

            let db = Database { 
                connection_string: "postgres://localhost:5432".into() 
            };
            // ---------------------

            // Now we execute the original function body
            // Because 'db' is defined above, the user's code can see it!
            let original_logic = || {
                #fn_block
            };
            
            original_logic();
        }
    };

    TokenStream::from(expanded)
}