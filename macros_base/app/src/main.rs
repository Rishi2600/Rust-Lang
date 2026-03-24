use my_macros::SqlTable;

#[derive(SqlTable, Debug)] // Added Debug so format!("{:?}") works
#[table("users")]
struct User {
    id: i32,
    name: String,
}

fn main() {
    let u = User { id: 42, name: "Rishi".to_string() };

    println!("Query:  {}", User::insert_sql());
    println!("Values: {:?}", u.values());
    
    // Pro move: Combine them!
    println!("\nExecuting: {} with params {:?}", User::insert_sql(), u.values());
}