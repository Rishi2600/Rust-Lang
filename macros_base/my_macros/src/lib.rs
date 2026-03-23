use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Getters)]
pub fn getters_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct { 
        fields: syn::Fields::Named(ref fields), .. 
    }) = ast.data { fields } else { panic!("Only named structs supported") };

    // Create a getter for each field
    let methods = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        quote! {
            pub fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    let expanded = quote! {
        impl #name {
            #(#methods)* // Expands the iterator into a list of methods
        }
    };
    TokenStream::from(expanded)
}