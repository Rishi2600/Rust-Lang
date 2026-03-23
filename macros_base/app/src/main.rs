fn main() {
    // Will panic on startup if these aren't set in the environment
    my_macros::require_envs!("DB_HOST", "API_KEY", "PORT");
    println!("Server starting...");
}