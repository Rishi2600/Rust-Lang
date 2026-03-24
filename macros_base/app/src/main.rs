use my_macros::inject_db;

#[inject_db]
fn get_users(db: &Database) {
    // This code 'sees' the db variable because the macro 
    // wrapped this block in a scope where 'db' exists!
    db.query("SELECT * FROM users");
}

fn main() {
    println!("--- Starting DI Project ---");
    
    // Call it with no arguments!
    get_users();
    
    println!("--- DI Project Finished ---");
}