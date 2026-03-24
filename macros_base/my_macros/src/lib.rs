use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, LitStr};

#[proc_macro_derive(SqlTable, attributes(table))]
pub fn sql_table_derive(input: TokenStream) -> TokenStream {
    // 1. Parse the struct definition
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // 2. Extract the table name from #[table("name")]
    // If missing, we default to the lowercase name of the struct
    let table_name = input.attrs.iter()
        .find(|a| a.path().is_ident("table"))
        .and_then(|a| a.parse_args::<LitStr>().ok())
        .map(|l| l.value())
        .unwrap_or_else(|| struct_name.to_string().to_lowercase());

    // 3. Look inside the struct to get the field names
    let fields = match input.data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => &fields.named,
                _ => panic!("SqlTable only works on structs with named fields"),
            }
        },
        _ => panic!("SqlTable only works on structs"),
    };

    let field_names: Vec<String> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect();

    // 4. Construct the SQL String
    let columns = field_names.join(", ");
    let placeholders = vec!["?"; field_names.len()].join(", ");
    let sql_query = format!("INSERT INTO {} ({}) VALUES ({});", table_name, columns, placeholders);

    // 5. Inject the new method into the User's struct
    let expanded = quote! {
        impl #struct_name {
            pub fn insert_sql() -> &'static str {
                #sql_query
            }
        }
    };

    TokenStream::from(expanded)
}