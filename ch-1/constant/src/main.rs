struct UserProfile {
    username: String,
    role: String,
}

enum UserSession {
    Guest,
    Authenticated(UserProfile),
    Banned { reason: String, expiry: String },
}

fn access_admin_panel(session: &UserSession) {
    match session {
        // Using "Nested Pattern Matching"
        UserSession::Authenticated(profile) if profile.role == "Admin" => {
            println!("Welcome, Master {}. Access granted.", profile.username);
        }
        UserSession::Authenticated(_) => {
            println!("Access Denied: Standard users cannot enter.");
        }
        UserSession::Banned { reason, .. } => {
            println!("Your account is restricted: {}", reason);
        }
        UserSession::Guest => {
            println!("Please log in to continue.");
        }
        //this
    }
}