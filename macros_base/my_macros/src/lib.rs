use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn require_envs(input: TokenStream) -> TokenStream {
    // Convert the input tokens (e.g., "PORT", "DB_URL") into an iterator of strings
    let envs: Vec<String> = input.to_string()
        .split(',')
        .map(|s| s.trim().replace("\"", ""))
        .collect();

    let checks = envs.iter().filter(|s| !s.is_empty()).map(|env| {
        quote! {
            if ::std::env::var(#env).is_err() {
                panic!("CRITICAL: Missing required environment variable: {}", #env);
            }
        }
    });

    let expanded = quote! {
        { #(#checks)* }
    };
    TokenStream::from(expanded)
}