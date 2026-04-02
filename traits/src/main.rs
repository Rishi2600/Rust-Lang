trait Animal {
    fn noise(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog { fn noise(&self) { println!("Woof"); } }
impl Animal for Cat { fn noise(&self) { println!("Meow"); } }

fn main() {
    // A list of different types that all follow the Animal contract
    let zoo: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in zoo {
        animal.noise();
    }
}