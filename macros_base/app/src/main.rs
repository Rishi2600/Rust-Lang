use my_macros::get_page;

#[get_page("postgres://localhost:5432")]
fn user_profile(user_id: i32) -> String {
    // Imagine this came from our SQL Mapper Project
    let username = "Rishi_The_Macro_Master";
    
    // Imagine this used our HTML DSL Project
    format!(
        "<h1>Profile for User #{}</h1><p>Username: {}</p>", 
        user_id, username
    )
}

fn main() {
    // We simulate a web request coming in for user 42
    let response = user_profile(42);
    
    println!("\nFinal Rendered Page:");
    println!("{}", response);
}

//leveraged proc macros to create a whole web framework handler