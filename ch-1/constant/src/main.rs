enum Payment {
    Pending,
    Processing { provider: String },
    Success { transaction_id: u32, amount: f64 },
    Failed { reason: String, code: i32 },
}

fn process_payment(payment: Payment) {
    match payment {
        // We only get access to transaction_id if the state is Success
        Payment::Success { transaction_id, amount } => {
            println!("Receipt sent for ID: {} (${})", transaction_id, amount);
        }
        Payment::Failed { reason, .. } => {
            println!("Alert: Payment failed due to {}", reason);
        }
        _ => println!("Payment is still in progress..."),
    }
}