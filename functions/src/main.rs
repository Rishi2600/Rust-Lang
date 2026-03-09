fn main() {
    let mystery_number = (1..)
        .map(|x| x * 10)
        .filter(|x| x % 3 == 0)
        .nth(100);
        
    println!("{:?}", mystery_number);
}