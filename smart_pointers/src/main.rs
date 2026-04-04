fn main() {
    // 5 lives on the Stack
    let x = 5; 

    // 5 lives on the Heap, 'y' is a pointer on the Stack
    let y = Box::new(5); 

    println!("y is {}", y); 
} // When 'y' goes out of scope, the Heap memory is automatically freed.