use my_macros::SqlTable;

// Example 1: Uses the attribute to specify the table name
#[derive(SqlTable)]
#[table("web_users")]
struct User {
    id: i32,
    username: String,
}

// Example 2: No attribute, so it will default to "product"
#[derive(SqlTable)]
struct Product {
    name: String,
    price: f64,
}

fn main() {
    println!("--- SQL MAPPER ---");
    
    // Call the generated methods
    println!("User SQL:    {}", User::insert_sql());
    println!("Product SQL: {}", Product::insert_sql());
}