fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // The Flow: Filter for even, square them, then collect into a new Vec
    let doubled_evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // Keep even numbers
        .map(|&x| x * x)         // Square them
        .collect();              // Finalize into a collection

    println!("{:?}", doubled_evens); // [4, 16, 36]
}