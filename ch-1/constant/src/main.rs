struct SmartConnection {
    id: String,
}

impl SmartConnection {
    fn send_data(&self, msg: &str) {
        println!("Sending via [{}]: {}", self.id, msg);
    }
}

// Custom drop logic runs automatically when the variable dies
impl Drop for SmartConnection {
    fn drop(&mut self) {
        println!("Disconnecting [{}] safely... Clear buffers... Freeing sockets.", self.id);
    }
}

fn main() {
    {
        let conn = SmartConnection { id: String::from("DB_CONN_1") };
        conn.send_data("SELECT * FROM users;");
    } // <-- `conn` goes out of scope HERE. `drop` runs immediately.

    println!("Back in main scope. The connection is already dead.");
}