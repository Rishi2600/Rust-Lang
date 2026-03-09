fn main() {
    let mut a = "Fire";
    let mut b = "Ice";

    // The classic swap magic
    (a, b) = (b, a);

    println!("a: {}, b: {}", a, b); // a: Ice, b: Fire
}