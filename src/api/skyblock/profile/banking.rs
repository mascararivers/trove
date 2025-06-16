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
pub struct EssenceStats {
    current: usize,    
}

#[derive(Deserialize, Debug)]
pub struct Essence {
    #[serde(rename = "WITHER")]
    wither: EssenceStats,
    #[serde(rename = "DRAGON")]
    dragon: EssenceStats,
    #[serde(rename = "SPIDER")]
    spider: EssenceStats,
    #[serde(rename = "UNDEAD")]
    undead: EssenceStats,
    #[serde(rename = "DIAMOND")]  
    diamond: EssenceStats,
    #[serde(rename = "GOLD")]
    gold: EssenceStats,
    #[serde(rename = "ICE")]
    ice: EssenceStats,
    #[serde(rename = "CRIMSON")]  
    crimson: EssenceStats,
}

#[derive(Deserialize, Debug)]
pub struct Currencies {
    coin_purse: usize,
    motes_purse: usize,
    essence: Essence,
}
