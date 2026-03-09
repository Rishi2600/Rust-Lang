fn spell_check() -> Option<String> {
    let word = Some("Abracadabra");
    // If word is None, the whole function returns None immediately.
    let result = word?.get(0..4)?; 
    
    Some(result.to_uppercase())
}

fn main() {
    println!("{:?}", spell_check()); // Some("ABRA")
}