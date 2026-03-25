use my_macros::trace;

#[trace]
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_numbers(5, 10);
    println!("The result is: {}", sum);
}