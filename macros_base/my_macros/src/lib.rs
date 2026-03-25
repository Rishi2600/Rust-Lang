use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_args = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_ret = &input_fn.sig.output;

    let expanded = quote! {
        #fn_vis fn #fn_name(#fn_args) #fn_ret {
            println!("[TRACE] Entering {}...", stringify!(#fn_name));
            
            // We use a block to capture the return value if there is one
            let result = (|| #fn_block)();
            
            println!("[TRACE] Exiting {}...", stringify!(#fn_name));
            result
        }
    };

    TokenStream::from(expanded)
}