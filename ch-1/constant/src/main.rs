use std::ops::Deref;

struct EncryptedString {
    raw_bytes: String,
}

impl Deref for EncryptedString {
    type Target = String;

    // Automatically routes references to the inner String
    fn deref(&self) -> &Self::Target {
        &self.raw_bytes
    }
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let secret = EncryptedString { raw_bytes: String::from("SecretPassword") };
    
    // Magic: We pass &secret (EncryptedString), but the compiler implicitly 
    // transforms it into &String via Deref coercion!
    print_length(&secret);
}