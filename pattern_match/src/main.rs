fn main() {
    println!("Hello, world!");
}
enum Portal {
    Fire,
    Ice,
    Void,
}

fn main() {
    let destination = Portal::Ice;

    match destination {
        Portal::Fire => println!("Bring a fan."),
        Portal::Ice => println!("Bring a coat."),
        Portal::Void => println!("Bring... nothing?"), // Must cover all cases!
    }
}