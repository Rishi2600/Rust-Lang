// Zero-Sized Type: Occupies 0 bytes of RAM
pub struct AdminPrivilegeToken;

pub struct SecureSystem;

impl SecureSystem {
    // This function physically cannot be called unless the caller possesses 
    // an instance of AdminPrivilegeToken.
    pub fn trigger_nuclear_launch(&self, _token: &AdminPrivilegeToken) {
        println!("🚀 Launch sequence initiated safely!");
    }
}

pub struct Authenticator;
impl Authenticator {
    pub fn login(&self, password: &str) -> Option<AdminPrivilegeToken> {
        if password == "correct_horse_battery_staple" {
            Some(AdminPrivilegeToken) // Hand out the zero-cost permission token
        } else {
            None
        }
    }
}

fn main() {
    let auth = Authenticator;
    let system = SecureSystem;

    if let Some(token) = auth.login("correct_horse_battery_staple") {
        // The token is passed by reference, costing nothing at runtime
        system.trigger_nuclear_launch(&token);
    }
}