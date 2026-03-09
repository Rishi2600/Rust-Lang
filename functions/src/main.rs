fn main() {
    let secret_char = 'g';

    match secret_char {
        'a'..='m' => println!("First half of the alphabet!"),
        'n'..='z' => println!("Second half of the alphabet!"),
        _ => println!("Not a lowercase letter!"),
    }
}