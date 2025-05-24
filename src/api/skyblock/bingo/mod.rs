use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Goal {
    id: String,
    name: String,
    lore: String,
    #[serde(rename = "fullLore")]
    full_lore: Vec<String>,
    tiers: Vec<usize>,
    progress: usize,
    #[serde(rename = "requiredAmount")]
    required_amount: usize,
}

#[derive(Debug, Deserialize)]
struct Bingo {
    success: bool,
    #[serde(rename = "lastUpdated")]
    last_updated: usize,
    id: usize,
    name: String,
    start: usize,
    end: usize,
    modifier: String,
    goals: Vec<Goal>,
}
