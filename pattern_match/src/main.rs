fn main() {
    let hand = [10, 2, 5, 8, 10];

    match hand {
        [10, .., 10] => println!("A pair of tens at the ends!"),
        [first, second, ..] => println!("Started with {} and {}", first, second),
        [] => println!("Empty hand."),
    }
}