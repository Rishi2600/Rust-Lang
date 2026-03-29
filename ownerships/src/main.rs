use std::cell::RefCell;

fn main() {
    // This variable is NOT 'mut'!
    let secret_box = RefCell::new(50);

    {
        // We can borrow it mutably even though the variable isn't mut!
        let mut val = secret_box.borrow_mut();
        *val += 10;
    } // Mutable borrow ends here

    println!("Value is now: {:?}", secret_box.borrow());
}