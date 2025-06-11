use serde::{ser, Deserialize};


#[derive(Deserialize, Debug)]
pub struct Completions {
    #[serde(rename = "NUCLEUS_RUNS")]
    nucleus_runs: usize,
    #[serde(rename = "REFINED_DARK_CACAO_TRUFFLE")]
    truffle: usize,
}

#[derive(Deserialize, Debug)]
pub struct Leveling {
    experience: usize,
    completions: Completions,
    completed_tasks: Vec<String>,
    category_expanded: bool,
    last_viewed_tasks: Vec<String>,
    highest_pet_score: usize,
    mining_fiesta_ores_mined: usize,
    fishing_festival_sharks_killed: usize,
    migrated: bool,
    migrated_completions_2: bool,
    claimed_talisman: bool,
    bop_bonus: String,
    selected_symbol: String,
    emblem_unlocks: Vec<String>,
    task_sort: String,
}