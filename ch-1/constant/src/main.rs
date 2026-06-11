trait Highlight {
    fn print_highlighted(&self);
}

// Blanket Implementation: Any type that implements std::fmt::Display 
// automatically gets the Highlight trait implemented for it instantly.
impl<T: std::fmt::Display> Highlight for T {
    fn print_highlighted(&self) {
        println!("✨ {} ✨", self);
    }
}

fn main() {
    // i32 natively implements Display, so it magically has print_highlighted() now!
    100.print_highlighted();
    
    // String natively implements Display, so it gets it too!
    String::from("Hello World").print_highlighted();
}