use my_macros::Describe;
use my_macros::log_call;

// THE MISSING PIECE: The trait definition
pub trait Describe {
    fn describe(&self);
}

#[derive(Describe)]
struct User {
    #[allow(dead_code)]
    name: String,
}

#[log_call]
fn do_work() {
    let u = User { name: "Alice".into() };
    u.describe();
}

fn main() {
    do_work();
}