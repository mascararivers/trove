use serde::{Deserialize, de};

#[derive(Deserialize, Debug)]
pub struct CrazyKloon {
    selected_colors: Vec<String>,
    talked: bool,
    hacked_terminals: Vec<String>,
    quest_complete: bool,
}

#[derive(Debug, Deserialize)]
pub struct Mirrorverse {
    visited_rooms: Vec<String>,
    upside_down_hard: bool,
    claimed_chest_items: Vec<String>,
    claimed_reward: bool,
}

#[derive(Deserialize, Debug)]
pub struct KatHouse {
    bin_collected_silverfish: usize,
    bin_collected_mosquito: usize,
    bin_collected_spider: usize,
}

#[derive(Deserialize, Debug)]
pub struct Glyph {
    claimed_wand: bool,
    current_glyph_delivered: bool,
    current_glyph_completed: bool,
    current_glyph: usize,
    completed: bool,
    claimed_bracelet: bool,
}

#[derive(Deserialize, Debug)]
pub struct WestVillage {
    crazy_kloon: CrazyKloon,
    mirrorverse: Mirrorverse,
    kat_house: KatHouse,
    glyphs: Vec<Glyph>,
}
