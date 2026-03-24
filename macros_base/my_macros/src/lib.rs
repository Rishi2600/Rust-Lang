use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr, LitInt};

#[proc_macro_derive(AutoCli, attributes(arg))]
pub fn auto_cli_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields"),
        },
        _ => panic!("Only structs"),
    };

    // We'll generate a piece of parsing logic for EVERY field
    let field_parsers = fields.iter().map(|f| {
    let name = &f.ident;
    let name_str = name.as_ref().unwrap().to_string();
    let long_flag = format!("--{}", name_str);
    
    // Check if the type name contains "Option"
    let type_str = quote!(#f.ty).to_string();
    let is_option = type_str.contains("Option");

    if is_option {
        quote! {
            let #name = std::env::args()
                .enumerate()
                .find(|(_, arg)| arg == #long_flag)
                .and_then(|(i, _)| std::env::args().nth(i + 1))
                .and_then(|val| val.parse().ok()); // Returns None if missing or parse fails
            }
        } else {
            quote! {
                let #name = std::env::args()
                    .enumerate()
                    .find(|(_, arg)| arg == #long_flag)
                    .and_then(|(i, _)| std::env::args().nth(i + 1))
                    .expect(&format!("Missing required argument: {}", #long_flag))
                    .parse()
                    .expect(&format!("Failed to parse argument: {}", #long_flag));
            }
        }
    });

    let field_names = fields.iter().map(|f| &f.ident);

    let expanded = quote! {
        impl #struct_name {
            pub fn parse() -> Self {
                #( #field_parsers )*

                Self {
                    #( #field_names ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}