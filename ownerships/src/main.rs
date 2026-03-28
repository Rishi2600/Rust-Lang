fn main() {
    let mut document = String::from("Draft 1");

    let reader1 = &document;
    let reader2 = &document; // ✅ Fine: multiple readers
    
    // let writer = &mut document; // ❌ ERROR: Cannot edit while readers are looking
    
    println!("Readers see: {} and {}", reader1, reader2);
    
    // Now that readers are "done" (out of scope), we can edit
    let writer = &mut document;
    writer.push_str(" - Final Version");
    println!("{}", writer);
}