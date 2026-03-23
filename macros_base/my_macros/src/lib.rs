use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, token, Ident, LitStr, Result, Token};

// 1. Define the recursive types
enum HtmlNode {
    Tag(HtmlTag),
    Text(LitStr),
}

struct HtmlTag {
    name: Ident,
    children: Vec<HtmlNode>,
}

// 2. Implement parsing for the Node (Choice: is it a String or a Tag?)
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(HtmlNode::Text(input.parse()?))
        } else {
            Ok(HtmlNode::Tag(input.parse()?))
        }
    }
}

// 3. Implement parsing for the Tag (Recursive step!)
impl Parse for HtmlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        
        let content;
        syn::braced!(content in input);
        
        let mut children = Vec::new();
        // Keep parsing nodes until the braces are empty
        while !content.is_empty() {
            children.push(content.parse()?);
        }
        
        Ok(HtmlTag { name, children })
    }
}

// 4. The Macro Entry Point
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlNode);
    let expanded = generate_node_code(&root);
    TokenStream::from(expanded)
}

// 5. Code Generation (Converts the tree into a format! call)
fn generate_node_code(node: &HtmlNode) -> proc_macro2::TokenStream {
    match node {
        HtmlNode::Text(lit) => quote! { #lit.to_string() },
        HtmlNode::Tag(tag) => {
            let name_str = tag.name.to_string();
            let child_codes: Vec<_> = tag.children.iter().map(generate_node_code).collect();
            
            // Generate a string like "{}{}{}" based on child count
            let placeholders = "{}".repeat(child_codes.len());
            
            quote! {
                format!(
                    "<{0}>{1}</{0}>", 
                    #name_str, 
                    format!(#placeholders, #(#child_codes),*)
                )
            }
        }
    }
}