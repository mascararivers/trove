use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contest {
    collected: usize,
    claimed_rewards: Option<bool>,
    claimed_position: Option<usize>,
    claimed_participants: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Medal {
    bronze: usize,
    silver: usize,
    gold: usize,
}

#[derive(Debug, Deserialize)]
pub struct Perks {
    double_drops: Option<usize>,
    farming_level_cap: Option<usize>,
    personal_bests: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UniqueBrackets {
    bronze: Vec<String>,
    silver: Vec<String>,
    gold: Vec<String>,
    diamond: Vec<String>,
    platinum: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PersonalBests {
    #[serde(rename = "SUGAR_CANE")]
    sugar_cane: Option<usize>,
    #[serde(rename = "CACTUS")]
    cactus: Option<usize>,
    #[serde(rename = "PUMPKIN")]
    pumpkin: Option<usize>,
    #[serde(rename = "NETHER_STALK")]
    nether_wart: Option<usize>,
    #[serde(rename = "INK_SACK:3")] // what the fuck is this supposed to be
    ink_sack: Option<usize>, // this api sucks i think im going to kill myself
    #[serde(rename = "MELON")]
    melon: Option<usize>,
    #[serde(rename = "WHEAT")]
    wheat: Option<usize>,
    #[serde(rename = "MUSHROOM_COLLECTION")] // bro
    shrooms: Option<usize>,
    #[serde(rename = "POTATO_ITEM")] // WHY THE FUCK could it not just be POTATO
    potato: Option<usize>,
    #[serde(rename = "CARROT_ITEM")]
    carrot: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct JacobsContest {
    medals_inv: Vec<Medal>,
    perks: Perks,
    contests: Vec<Contest>,
    talked: bool,
    unique_brackets: UniqueBrackets,
    migration: bool,
    personal_bests: PersonalBests,
}
