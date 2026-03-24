use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr};

#[proc_macro_derive(AutoCli, attributes(arg))]
pub fn auto_cli_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Only named fields supported"),
        },
        _ => panic!("Only structs supported"),
    };

    let field_parsers = fields.iter().map(|f| {
        let name = &f.ident;
        let name_str = name.as_ref().unwrap().to_string();
        let long_flag = format!("--{}", name_str);
        
        // 1. Parse Attribute for Short Flag
        let mut short_flag_val = None;
        let _ = f.attrs.iter().find(|a| a.path().is_ident("arg")).map(|a| {
            let _ = a.parse_nested_meta(|meta| {
                if meta.path.is_ident("short") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    short_flag_val = Some(format!("-{}", s.value()));
                }
                Ok(())
            });
        });

        // 2. Determine Type Logic
        let type_str = quote!(#f.ty).to_string().replace(" ", "");
        let is_bool = type_str == "bool";
        let is_option = type_str.contains("Option<");

        // Convert the Option<String> to something we can use inside quote!
        let short_flag_expr = match short_flag_val {
            Some(s) => quote! { Some(#s) },
            None => quote! { None::<&str> },
        };

        if is_bool {
            quote! {
                let #name = std::env::args().any(|arg| arg == #long_flag || Some(arg.as_str()) == #short_flag_expr);
            }
        } else if is_option {
            quote! {
                let #name = std::env::args()
                    .enumerate()
                    .find(|(_, arg)| arg == #long_flag || Some(arg.as_str()) == #short_flag_expr)
                    .and_then(|(i, _)| std::env::args().nth(i + 1))
                    .and_then(|val| val.parse().ok());
            }
        } else {
            quote! {
                let #name = std::env::args()
                    .enumerate()
                    .find(|(_, arg)| arg == #long_flag || Some(arg.as_str()) == #short_flag_expr)
                    .and_then(|(i, _)| std::env::args().nth(i + 1))
                    .expect(&format!("Missing required argument: {} or {:?}", #long_flag, #short_flag_expr))
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
                Self { #( #field_names ),* }
            }
        }
    };

    TokenStream::from(expanded)
}