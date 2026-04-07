use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    let text = "apple banana apple orange apple banana";

    for word in text.split_whitespace() {
        // .entry() returns an Enum! 
        // If it's Vacant, it inserts 0. Then it returns a mutable reference to the value.
        let count = scores.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", scores); // {"apple": 3, "banana": 2, "orange": 1}
}