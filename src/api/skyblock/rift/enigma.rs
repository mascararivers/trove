use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Enigma {
    bought_cloak: bool,
    found_souls: Vec<String>,
    claimed_bonus_index: usize,
}
