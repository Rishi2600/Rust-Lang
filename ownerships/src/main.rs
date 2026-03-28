fn main() {
    let s1 = String::from("The Ancient Scroll");
    
    // Ownership of the data MOVES from s1 to s2
    let s2 = s1; 

    // println!("{}", s1); // ❌ ERROR: s1 is now "empty" (invalid)
    println!("{}", s2);    
}