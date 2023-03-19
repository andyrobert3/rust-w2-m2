mod transaction;

use transaction::*;

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
