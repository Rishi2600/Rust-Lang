fn main() {
    let some_option = Some("Golden Idol");

    // "If this matches Some(x), bind x and run this block"
    if let Some(treasure) = some_option {
        println!("Found the {}!", treasure);
    } 
    // No need to handle 'None' if we don't want to.
}