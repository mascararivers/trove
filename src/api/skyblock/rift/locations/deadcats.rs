use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Montezuma {
    uuid: Option<String>,
    #[serde(rename = "uniqueId")]
    unique_id: String,
    #[serde(rename = "type")]
    montezuma_type: String,
    exp: usize,
    active: bool,
    tier: String,
    #[serde(rename = "heldItem")]
    held_item: Option<String>,
    #[serde(rename = "candyUsed")]
    candy_used: usize,
    skin: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DeadCats {
    talked_to_jacquelle: bool,
    picked_up_detector: bool,
    found_cats: Vec<String>,
    unlocked_pet: bool,
    montezuma: Montezuma,
}
