fn main() {
    let condition = true;

    let number: &str = if condition { "5" } else { "six" };

    println!("The value of number is: {number}");
}