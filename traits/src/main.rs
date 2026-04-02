fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // This "Lazy" chain does nothing until .collect() is called
    let result: Vec<i32> = numbers.into_iter()
        .filter(|x| x % 2 != 0) // Keep odds
        .map(|x| x * x)         // Square them
        .take(2)                // Only take the first two
        .collect();             // Turn back into a Vec

    println!("{:?}", result); // [1, 9]
}