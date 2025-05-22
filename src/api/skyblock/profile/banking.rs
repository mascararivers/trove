
#[derive(Deserialize, Debug)]
struct BankTransaction {
    timestamp: usize,
    action: String,
    initiator_name: String,
    amount: usize,
}

#[derive(Deserialize, Debug)]
struct Banking {
    balance: usize,
    transactions: Vec<BankTransaction>, 
}