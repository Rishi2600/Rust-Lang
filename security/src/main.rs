fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    // Superpower: Combinators (.map, .and_then, .unwrap_or)
    let display = result
        .map(|n| n * 2.0)
        .unwrap_or(0.0);

    println!("Result: {}", display);
}