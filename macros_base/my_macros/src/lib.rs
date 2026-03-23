use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, token, Ident, LitStr, Result, Token};

// 1. DATA STRUCTURES (Must be defined before they are used)
enum HtmlNode {
    Tag(HtmlTag),
    Text(LitStr),
}

struct HtmlAttr {
    key: Ident,
    value: LitStr,
}

struct HtmlTag {
    name: Ident,
    attributes: Vec<HtmlAttr>,
    children: Vec<HtmlNode>,
}

// 2. PARSING LOGIC
impl Parse for HtmlNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            Ok(HtmlNode::Text(input.parse()?))
        } else {
            Ok(HtmlNode::Tag(input.parse()?))
        }
    }
}

impl Parse for HtmlTag {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        
        // Parse Attributes: (key="value", ...)
        let mut attributes = Vec::new();
        if input.peek(token::Paren) {
            let attr_content;
            syn::parenthesized!(attr_content in input);
            while !attr_content.is_empty() {
                let key: Ident = attr_content.parse()?;
                attr_content.parse::<Token![=]>()?;
                let value: LitStr = attr_content.parse()?;
                attributes.push(HtmlAttr { key, value });
                if attr_content.peek(Token![,]) {
                    attr_content.parse::<Token![,]>()?;
                }
            }
        }

        // Parse Children: { ... }
        let content;
        syn::braced!(content in input);
        let mut children = Vec::new();
        while !content.is_empty() {
            children.push(content.parse()?);
        }
        
        Ok(HtmlTag { name, attributes, children }) // Fixed the missing field error here
    }
}

// 3. CODE GENERATION
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as HtmlNode);
    let expanded = generate_node_code(&root);
    TokenStream::from(expanded)
}

fn generate_node_code(node: &HtmlNode) -> proc_macro2::TokenStream {
    match node {
        HtmlNode::Text(lit) => quote! { #lit.to_string() },
        HtmlNode::Tag(tag) => {
            let name_str = tag.name.to_string();
            
            let mut attr_str = String::new();
            for attr in &tag.attributes {
                attr_str.push_str(&format!(" {}=\"{}\"", attr.key, attr.value.value()));
            }

            let child_codes: Vec<_> = tag.children.iter().map(generate_node_code).collect();
            let placeholders = "{}".repeat(child_codes.len());
            
            quote! {
                format!(
                    "<{0}{1}>{2}</{0}>", 
                    #name_str, 
                    #attr_str,
                    format!(#placeholders, #(#child_codes),*)
                )
            }
        }
    }
}