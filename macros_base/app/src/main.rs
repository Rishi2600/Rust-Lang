use my_macros::inject_db;

#[inject_db("postgres://production-db:5432")]
fn sync_prod(db: &Database) {
    db.query("SELECT * FROM users");
}

#[inject_db("sqlite://local.db")]
fn sync_local(db: &Database) {
    db.query("INSERT INTO logs VALUES ('sync_started')");
}

fn main() {
    println!("Starting Multi-DB Sync...");
    
    sync_prod();
    sync_local();
    
    println!("Sync Complete.");
}