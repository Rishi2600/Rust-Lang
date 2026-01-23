use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    println!("Guess the Number");

    let random_numebr = rand::thread_rng().gen_range(1..=10);

    println!("The random numer is {random_number}");

    loop {
        println!("PUt in the number");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read the line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::less => println!("Number guessed is lesser"),
            Ordering::Greater => println!("Number guessed is greater"),
            Ordering::Equal => {
                println!("Correct Guess. You Win!");
                break:
            }
        };
    }
}