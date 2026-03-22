use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(JsonPlan, attributes(rename))] // 'attributes(rename)' registers the helper
pub fn json_plan_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    // We only care about structs with named fields for this example
    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            &fields.named
        } else { panic!("JsonPlan only works on structs with named fields"); }
    } else { panic!("JsonPlan only works on structs"); };

    let mut field_logics = Vec::new();

    for field in fields {
        let field_name = &field.ident;
        
        // Search for our #[rename = "new_name"] attribute
        let mut display_name = quote! { stringify!(#field_name) };
        
        for attr in &field.attrs {
            if attr.path().is_ident("rename") {
                // Parse the value inside: #[rename = "XYZ"]
                if let Ok(syn::ExprParen { expr, .. }) = attr.parse_args::<syn::ExprParen>() {
                     display_name = quote! { #expr };
                } else if let Ok(lit) = attr.parse_args::<syn::LitStr>() {
                     display_name = quote! { #lit };
                }
            }
        }

        field_logics.push(quote! {
            println!("Field: {} (Internal: {})", #display_name, stringify!(#field_name));
        });
    }

    let expanded = quote! {
        impl JsonPlan for #name {
            fn output_plan(&self) {
                println!("--- Mapping Plan for {} ---", stringify!(#name));
                #(#field_logics)*
            }
        }
    };

    TokenStream::from(expanded)
}