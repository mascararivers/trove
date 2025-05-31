use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct BankTransaction {
    timestamp: usize,
    action: String,
    initiator_name: String,
    amount: usize,
}

#[derive(Deserialize, Debug)]
pub struct Banking {
    balance: usize,
    transactions: Vec<BankTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct Currencies {
    coin_purse: usize
}