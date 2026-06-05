#[derive(Debug)]
struct Token<'a> {
    // The 'a tells Rust: "This slice cannot outlive the raw source text string"
    kind: &'a str,
    value: &'a str,
}

fn tokenize<'a>(source: &'a str) -> Vec<Token<'a>> {
    let mut tokens = Vec::new();
    
    // Split the text by spaces
    for word in source.split_whitespace() {
        if word.starts_with('$') {
            tokens.push(Token { kind: "VARIABLE", value: &word[1..] });
        } else {
            tokens.push(Token { kind: "TEXT", value: word });
        }
    }
    tokens
}

fn main() {
    // 100% stack allocated strings/slices
    let source_text = "Welcome to Rust $user_name"; 
    
    let tokens = tokenize(source_text);
    println!("Tokens: {:#?}", tokens);
}