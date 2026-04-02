struct Seconds(u32);
struct Minutes(u32);

impl From<Minutes> for Seconds {
    fn from(m: Minutes) -> Self {
        Seconds(m.0 * 60)
    }
}

fn main() {
    let mins = Minutes(5);
    
    // We can use .into() because we implemented From!
    let secs: Seconds = mins.into(); 
    println!("Total seconds: {}", secs.0);
}