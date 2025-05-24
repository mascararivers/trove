use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Upgrades {
    speed: usize,
    multi_drop: usize,
    fuel_cap: usize,
    organic_matter_cap: usize,
    cost_reduction: usize,
}

#[derive(Debug, Deserialize)]
pub struct ComposterData {
    organic_matter: usize,
    fuel_units: usize,
    compost_units: usize,
    compost_items: usize,
    conversion_ticks: usize,
    last_save: usize,
    upgrades: Vec<Upgrades>,
}
