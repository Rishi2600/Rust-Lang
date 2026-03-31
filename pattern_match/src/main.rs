struct Account {
    status: Status,
}

enum Status {
    Active(u32), // Credits remaining
    Suspended,
}

fn main() {
    let user_account = Some(Account { status: Status::Active(100) });

    // We "dig" through the Option, then the Struct, then the Enum
    match user_account {
        Some(Account { status: Status::Active(amount) }) if amount > 50 => {
            println!("User is wealthy with {} credits!", amount);
        }
        Some(Account { status: Status::Suspended }) => println!("Access denied."),
        _ => println!("No account found or low balance."),
    }
}