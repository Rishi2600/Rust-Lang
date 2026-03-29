use std::rc::Rc;

fn main() {
    let scroll = Rc::new(String::from("Forbidden Knowledge"));

    let apprentice_a = Rc::clone(&scroll); // Increases count to 2
    let _apprentice_b = Rc::clone(&scroll); // Increases count to 3

    println!("Apprentice A reads: {}", apprentice_a);
    println!("Owners count: {}", Rc::strong_count(&scroll));
} // Count drops as apprentices go out of scope. Data dies at count 0.