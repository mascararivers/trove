use serde::{Deserialize, de};

use crate::api::skyblock::rift::locations::dreadfarm;

#[derive(Debug, Deserialize)]
pub struct DungeonFloors {
    #[serde(rename = "0")]
    entrance: usize,
    #[serde(rename = "1")]
    floor_1: usize,
    #[serde(rename = "2")]
    floor_2: usize,
    #[serde(rename = "3")]
    floor_3: usize,
    #[serde(rename = "4")]
    floor_4: usize,
    #[serde(rename = "5")]
    floor_5: usize,
    #[serde(rename = "6")]
    floor_6: usize,
    #[serde(rename = "7")]
    floor_7: usize,
    total: Option<usize>, // apparently some requests dont return a total for whatever reason because fuck you that's why
    best: Option<usize> // pb
}

#[derive(Debug, Deserialize)]
pub struct Run {
    timestamp: usize,
    score_exploration: usize,
    score_speed: usize,
    score_skill: usize,
    score_bonus: usize,
    dungeon_class: String,
    teammates: Vec<String>, // String representing player UUIDs
    elapsed_time: usize,
    damage_dealt: usize,
    deaths: usize,
    mobs_killed: usize,
    secrets_found: usize,
    damage_mitigated: usize,
    ally_healing: usize,
}

#[derive(Debug, Deserialize)]
pub struct BestRun {
    #[serde(rename = "0")]
    entrance: Run,
    #[serde(rename = "1")]
    floor_1: Run,
    #[serde(rename = "2")]
    floor_2: Run,
    #[serde(rename = "3")]
    floor_3: Run,
    #[serde(rename = "4")]
    floor_4: Run,
    #[serde(rename = "5")]
    floor_5: Run,
    #[serde(rename = "6")]
    floor_6: Run,
    #[serde(rename = "7")]
    floor_7: Run,
}

#[derive(Debug, Deserialize)]
pub struct Catacombs {
    times_played: DungeonFloors,
    experience: usize,
    best_score: DungeonFloors, // Duping structs because im not making another one
    mobs_killed: DungeonFloors,
    most_mobs_killed: DungeonFloors,
    most_healing: DungeonFloors,
    tier_completions: DungeonFloors,
    milestones: DungeonFloors,
    fastest_time: DungeonFloors,
    best_runs: Vec<BestRun>,
    watcher_kills: DungeonFloors,
    highest_tier_completed: usize,
    most_damage_archer: DungeonFloors,
    most_damage_berserk: DungeonFloors,
    most_damage_mage: DungeonFloors,
    most_damage_tank: DungeonFloors,
    most_damage_healer: DungeonFloors,
    fastest_time_s_plus: DungeonFloors,
    fastest_time_s: DungeonFloors,
}

#[derive(Debug, Deserialize)]
pub struct MasterCatacombs {
    tier_completions: DungeonFloors,
    milestone_completions: DungeonFloors,
    highest_tier_completed: usize,
    fastest_time: DungeonFloors,
    fastest_time_s_plus: DungeonFloors,
    best_runs: Vec<BestRun>,
    best_score: DungeonFloors,
    mobs_killed: DungeonFloors,
    most_mobs_killed: DungeonFloors,
    most_damage_mage: DungeonFloors,
    most_healing: DungeonFloors,
    fastest_time_s: DungeonFloors,
    most_damage_tank: DungeonFloors,
    most_damage_healer: DungeonFloors,
    most_damage_archer: DungeonFloors,
    most_damage_berserk: DungeonFloors,
    times_played: Option<DungeonFloors>, // Request returned this as {} dunno why
    watcher_kills: Option<DungeonFloors>, // Same as above
}

#[derive(Debug, Deserialize)]
pub struct DungeonType {
    catacombs: Catacombs,
    master_catacombs: MasterCatacombs,
}

#[derive(Debug, Deserialize)]
pub struct SubPlayerClass {
    experience: usize,
}

#[derive(Debug, Deserialize)]
pub struct PlayerClass {
    healer: SubPlayerClass,
    mage: SubPlayerClass,
    berserk: SubPlayerClass,
    archer: SubPlayerClass,
    tank: SubPlayerClass,
}

#[derive(Debug, Deserialize)]
pub struct DungeonJournalEntries {
    karylles_diary: Vec<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DungeonJournal {
    journal_entries: Vec<DungeonJournalEntries>,
}

#[derive(Debug, Deserialize)]
pub struct DailyRuns {
    current_day_stamp: usize,
    completed_runs_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct TreasuresParticipant {
    player_uuid: String,
    display_name: String,
    class_milestone: usize,
}

#[derive(Debug, Deserialize)]
pub struct TreasuresRun {
    run_id: String,
    completion_ts: usize, // I don't know why they couldn't have called this completion_timestamp
    dungeon_type: String,
    dungeon_tier: usize,
    participants: Vec<TreasuresParticipant>,
}

#[derive(Debug, Deserialize)]
pub struct ChestRewards {
    rewards: Vec<String>, // Why the fuck is this even necessary why nest this shit im gonna kill myself
    rolled_rng_meter_randomly: bool, // ???
}

#[derive(Debug, Deserialize)]
pub struct Chest {
    run_id: String,
    chest_id: String,
    treasure_type: String,
    rewards: ChestRewards,
    quality: usize,
    shiny_eligible: bool,
    paid: bool,
    rerolls: usize // should just be rerolled: bool because you cant reroll more than once but fuck logic
}

#[derive(Debug, Deserialize)]
pub struct Treasures {
    runs: Vec<TreasuresRun>,
    chests: Vec<Chest>,
}

#[derive(Debug, Deserialize)]
pub struct RaceSettings {
    selected_race: String,
    selected_setting: String,
    runback: bool
}

#[derive(Deserialize, Debug)]
pub struct Dungeons {
    dungeon_types: Vec<DungeonType>,
    player_classes: Vec<PlayerClass>,
    dungeon_journal: DungeonJournal,
    dungeons_blah_blah: Vec<String>, // Yes, this is what its called in the API
    selected_dungeon_class: String,
    daily_runs: DailyRuns,
    treasures: Treasures,
    dungeon_hub_race_settings: RaceSettings,
    secrets: usize,
    last_dungeon_run: String
}
