use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Slot {
    health: usize,
    defense: usize,
    walk_speed: usize,
    strength: usize,
    critical_damage: usize,
    critical_chance: usize,
    attack_speed: usize,
    intelligence: usize,
}

#[derive(Deserialize, Debug)]
pub struct Tuning {
    slot_0: Slot,
    highest_unlocked_slot: usize,
    slot_1: Slot,
    slot_2: Slot,
    refund_1: bool,
}

#[derive(Deserialize, Debug)]
pub struct AccessoryBagStorage {
    tuning: Tuning,
    unlocked_powers: Vec<String>,
    selected_power: String,
    bag_upgrades_purchased: usize,
    highest_magical_power: usize,
}