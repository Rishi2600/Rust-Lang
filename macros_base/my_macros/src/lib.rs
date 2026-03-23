use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Ident, Token, parse::{Parse, ParseStream}, Result};

// 1. Define a "Node" structure to hold our parsed data
struct HtmlNode {
    tag: Ident,
    content: LitStr,
}

// 2. Implement "Parse" so 'syn' knows how to read our custom syntax
// Expected: tag_name { "string content" }
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        let tag: Ident = input.parse()?;
        
        // Look for braces { ... }
        let content;
        syn::braced!(content in input);
        let lit: LitStr = content.parse()?;
        
        Ok(HtmlNode { tag, content: lit })
    }
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as HtmlNode);
    let tag_str = node.tag.to_string();
    let content_str = node.content.value();

    // 3. Generate the actual String-building code
    let result = format!("<{}>{}</{}>", tag_str, content_str, tag_str);

    let expanded = quote! {
        #result
    };
    TokenStream::from(expanded)
}