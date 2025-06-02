use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct JadeCrystal {
    state: Option<String>,
    total_found: Option<usize>,
    total_placed: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct AmberCrystal {
    state: Option<String>,
    total_found: Option<usize>,
    total_placed: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct AmethystCrystal {
    state: Option<String>,
    total_found: Option<usize>,
    total_placed: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct SapphireCrystal {
    state: Option<String>,
    total_found: Option<usize>,
    total_placed: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub struct TopazCrystal {
    state: Option<String>,
    total_found: Option<usize>,
    total_placed: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct Crystals {
    jade_crystal: Option<JadeCrystal>, 
    amber_crystal: Option<AmberCrystal>, 
    amethyst_crystal: Option<AmethystCrystal>, 
    sapphire_crystal: Option<SapphireCrystal>,  
    topaz_crystal: Option<TopazCrystal>, 
}

#[derive(Deserialize, Debug)]
struct Node {
    mining_madness: Option<usize>,
}

#[derive(Deserialize, Debug)]
struct Dwarven;
#[derive(Deserialize, Debug)]
struct Precursor {
    parts_delivered: usize,
}
#[derive(Deserialize, Debug)]
struct Goblin {
    king_quest_active: bool,
    king_quests_completed: usize,
}

#[derive(Deserialize, Debug)]
struct Biomes {
    dwarven: Option<Dwarven>,

}

#[derive(Deserialize, Debug)]
struct MiningCore {
    nodes: Vec<Node>,
    tokens_spent: usize,
    retroactive_tier2_token: bool,
    powder_spent_mithril: usize,
    powder_gemstone: usize,
    powder_gemstone_total: usize,
    greater_mines_last_access: usize,
    hotm_migrator_tree_reset_send_message: bool,
    crystals: Vec<Crystals>,
}