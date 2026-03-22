use proc_macro::TokenStream;
use quote::quote;

#[proc_macro] // <--- This must be exactly this for sql_check!()
pub fn sql_check(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    // Basic validation logic
    if !input_str.contains("SELECT") {
        panic!("Invalid SQL: Queries must start with SELECT!");
    }

    // Return the string as a literal so it can be assigned to a variable
    let expanded = quote! { #input_str };
    TokenStream::from(expanded)
}