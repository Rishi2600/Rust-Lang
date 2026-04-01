enum ServerError {
    Timeout(u32),
    ConnectionLost,
    AuthFailed,
    Maintenance(String),
}

fn main() {
    let status = ServerError::Timeout(500);

    match status {
        // Multiple patterns, one result
        ServerError::Timeout(_) | ServerError::ConnectionLost => {
            println!("Retrying connection...");
        }
        ServerError::AuthFailed | ServerError::Maintenance(_) => {
            println!("Please log in again later.");
        }
    }
}