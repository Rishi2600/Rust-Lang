use std::mem;

fn main() {
    let mut x = String::from("Alice");
    let mut y = String::from("Bob");

    // Swaps the contents of two memory locations without reallocating
    mem::swap(&mut x, &mut y);

    println!("x: {}, y: {}", x, y); // x: Bob, y: Alice
}