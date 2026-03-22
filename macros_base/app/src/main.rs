use my_macros::log_call;

#[log_call]
fn add_numbers(a: i32, b: i32) -> i32 {
    println!("Calculating...");
    a + b
}

fn main() {
    let sum = add_numbers(10, 20);
    println!("Result: {}", sum);
}