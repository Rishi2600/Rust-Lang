fn main() {
    let coordinates = vec![(0, 0), (10, 5), (20, 10)];

    // Destructuring the tuple right in the loop signature
    for (x, y) in coordinates {
        println!("Point is at {}, {}", x, y);
    }
}