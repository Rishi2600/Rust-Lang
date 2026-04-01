fn main() {
    let data = vec![Some(10), None, Some(20), Some(30), None];

    // .flatten() essentially pattern matches under the hood!
    // It looks for Some(x) and discards None.
    let numbers: Vec<i32> = data.into_iter().flatten().collect();

    println!("Only the 'Some' values: {:?}", numbers);
}