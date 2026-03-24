use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr};

#[proc_macro_derive(SqlTable, attributes(table))]
pub fn sql_table_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // 1. Get Table Name (Existing Logic)
    let table_name = input.attrs.iter()
        .find(|a| a.path().is_ident("table"))
        .and_then(|a| a.parse_args::<syn::LitStr>().ok())
        .map(|l| l.value())
        .unwrap_or_else(|| struct_name.to_string().to_lowercase());

    // 2. Get Fields
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Named fields only"),
        },
        _ => panic!("Structs only"),
    };

    // 3. Prepare identifiers for the "values" method
    let field_idents: Vec<&syn::Ident> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    
    let field_names_str: Vec<String> = field_idents.iter()
        .map(|i| i.to_string())
        .collect();

    // 4. Build the SQL String
    let columns = field_names_str.join(", ");
    let placeholders = vec!["?"; field_names_str.len()].join(", ");
    let sql_query = format!("INSERT INTO {} ({}) VALUES ({});", table_name, columns, placeholders);

    // 5. Generate Code
    let expanded = quote! {
        impl #struct_name {
            pub fn insert_sql() -> &'static str {
                #sql_query
            }

            pub fn values(&self) -> Vec<String> {
                // This converts every field into a String for a quick demo
                vec![
                    #( format!("{:?}", self.#field_idents) ),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}