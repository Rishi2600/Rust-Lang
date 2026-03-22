use my_macros::sql_check;

fn main() {
    // This compiles fine:
    let query = sql_check!("SELECT * FROM users");
    println!("Query: {}", query);

    // UNCOMMENT THIS TO BREAK THE BUILD:
    // let bad_query = sql_check!("DROP TABLE users"); 
}