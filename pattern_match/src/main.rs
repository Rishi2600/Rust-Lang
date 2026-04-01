fn main() {
    let mut name = Some(String::from("Rishi"));

    match name {
        // 'ref mut n' says: "Don't move the string, just give me a mutable pointer to it"
        Some(ref mut n) => {
            n.push_str(" the Architect");
            println!("Modified: {}", n);
        },
        None => (),
    }

    // Because we used 'ref mut', 'name' is still valid here!
    println!("Original is still alive: {:?}", name);
}