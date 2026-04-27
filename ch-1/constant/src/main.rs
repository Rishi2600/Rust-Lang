enum ConnectionState {
    Disconnected,
    Connecting(u32), // Attempt count
    Connected { session_id: String, ip: String },
    Failed(String),  // Error message
}

fn handle_state(state: &ConnectionState) {
    match state {
        ConnectionState::Connected { session_id, .. } => {
            println!("Logged in as {}", session_id);
        }
        ConnectionState::Connecting(attempt) if *attempt > 3 => {
            println!("Connection is taking a while (Attempt {})...", attempt);
        }
        _ => println!("Status: Not fully connected."),
    }
}