fn main() {
    let s1 = String::from("Library Book");

    // We pass a reference (&), not the value itself
    print_length(&s1); 

    // s1 still owns the data, so we can use it here!
    println!("I still have the '{}'", s1);
}

fn print_length(s: &String) {
    println!("The book is {} chars long", s.len());
}