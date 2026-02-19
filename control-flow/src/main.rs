fn main() {
    let mut found = false;
    let number = 6;

    if number % 4 == 0 {
        println!("number is devisible by 4");
        found = true;
    }

    if number % 3 == 0 {
        println!("number is devisible by 3");
        found = true;
    }

    if number % 2 == 0 {
        println!("number is devisible by 2");
        found = true;
    }

    if found == false {
        println!("number is not devisible by either 2, 3, or 4");
    }
}