use serde::{de, ser, Deserialize};

#[derive(Deserialize, Debug)]
pub struct PetRuleData {
    boss: String,
    category: String,
    island: String,
    entity_type: String,
    slot: String, // ??
}

#[derive(Deserialize, Debug)]
pub struct PetRule {
    uuid: String,
    id: String,
    name: String,
    #[serde(rename = "uniqueId")]
    unique_id: String,
    exceptions: Vec<String>,
    disabled: bool,
    data: PetRuleData
}

#[derive(Deserialize, Debug)]
pub struct AutoPet {
    rules_limit: usize,
    rules: Vec<PetRule>,
}

#[derive(Deserialize, Debug)]
pub struct PetCare {
    coins_spent: usize,
    pet_types_sacrificed: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Pet {
    uuid: String,
    #[serde(rename = "uniqueId")]
    unique_id: String,
    #[serde(rename = "type")]
    pet_type: String,
    exp: usize,
    active: bool,
    tier: String,
    #[serde(rename = "heldItem")]
    held_item: String,
    #[serde(rename = "candyUsed")]
    candy_used: usize,
    skin: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PetData {
    pet_care: PetCare,
    autopet: AutoPet,
    migrated: bool,
    migrated_2: bool,
    pets: Vec<Pet>,
}