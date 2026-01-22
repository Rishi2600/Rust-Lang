use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main () {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please put in a number");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Lesser number guessed"),
            Ordering::Greater => println!("Greater number guessed"),
            Ordering::Equal => {
                println!("Correct number guessed. You win!");
                break;
            }
        }
    }
    
}