fn withdrawal() {
    println!("User requested a withdrawal")
}

fn deposit() {
    println!("User requested a deposit")
}

fn transfer() {
    println!("User requested a transfer")
}

enum TransactionType {
    Withdrawal,
    Deposit,
    Transfer,
}

fn main() {
    let transaction = TransactionType::Withdrawal;
    match transaction {
        TransactionType::Withdrawal => withdrawal(),
        TransactionType::Deposit => deposit(),
        TransactionType::Transfer => transfer(),
    }
}
