use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumStr)]
pub fn enum_str_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let variants = if let syn::Data::Enum(data) = ast.data {
        data.variants
    } else { panic!("EnumStr only works on Enums!"); };

    // Map each variant to: Self::Variant => "Variant"
    let match_arms = variants.iter().map(|v| {
        let variant_name = &v.ident;
        quote! {
            Self::#variant_name => stringify!(#variant_name)
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };
    TokenStream::from(expanded)
}