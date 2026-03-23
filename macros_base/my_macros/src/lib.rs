use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn retry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);
    
    // Parse the attribute arguments (e.g., #[retry(3)])
    let retries: usize = attr.to_string().parse().unwrap_or(3); 
    
    let sig = &input.sig;
    let block = &input.block;

    let expanded = quote! {
        #sig {
            let mut attempts = 0;
            loop {
                // Execute the block and match the Result
                match (|| #block)() {
                    Ok(val) => break Ok(val),
                    Err(e) if attempts < #retries => {
                        attempts += 1;
                        println!("Retrying... (Attempt {})", attempts);
                    },
                    Err(e) => break Err(e),
                }
            }
        }
    };
    TokenStream::from(expanded)
}