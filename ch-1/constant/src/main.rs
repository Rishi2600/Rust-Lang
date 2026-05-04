use std::cell::RefCell;

struct UIElement {
    id: i32,
    // RefCell lets us change 'clicks' even if the struct is 'immutable'
    clicks: RefCell<u32>, 
}

fn main() {
    let button = UIElement {
        id: 1,
        clicks: RefCell::new(0),
    };

    // Note: 'button' is NOT marked 'mut'
    {
        let mut tracker = button.clicks.borrow_mut();
        *tracker += 1;
    }

    println!("Button {} clicked {} times", button.id, button.clicks.borrow());
}