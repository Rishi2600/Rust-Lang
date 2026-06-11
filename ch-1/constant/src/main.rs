// 1. Define a brand new trait
pub trait IsEven {
    fn is_even(&self) -> bool;
}

// 2. Implement your trait for an existing type from the Standard Library
impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

fn main() {
    let number = 42;
    // Magic: i32 now natively has the .is_even() method!
    println!("Is {} even? {}", number, number.is_even());
}