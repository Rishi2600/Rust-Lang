use my_macros::html;

fn main() {
    // This looks like Rust, but it's our custom HTML language!
    let page = html!(div { "Hello from the DSL!" });
    
    println!("Generated HTML: {}", page);
}