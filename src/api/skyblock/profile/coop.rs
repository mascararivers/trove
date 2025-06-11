use super::{Timestamp, banking::Currencies};
use crate::api::skyblock::{
    dungeons::Dungeons,
    garden::{data::jacobs_contest::{self, JacobsContest}, GardenPlayerData},
    mining::GlacitePlayerData, misc::{accessories::AccessoryBagStorage, events::Events, pets::PetData}, rift::Rift,
};
use serde::{Deserialize, de};

#[derive(Deserialize, Debug)]
struct Perk {
    catacombs_boss_luck: usize,
    catacombs_intelligence: usize,
    forbidden_blessing: usize,
    
    catacombs_looting: usize,
    catacombs_strength: usize,
}

#[derive(Deserialize, Debug)]
struct PotionEffect;

#[derive(Deserialize, Debug)]
struct SkillExperience {
    skill: String,
    experience: usize,
}

#[derive(Deserialize, Debug)]
struct PlayerData {
    visited_zones: Vec<String>,
    last_death: Option<Timestamp>,
    perks: Vec<Perk>,
    active_effects: Vec<PotionEffect>,
    paused_effects: Vec<PotionEffect>,
    reaper_peppers_eaten: Option<usize>,
    temp_stat_buffs: Vec<String>,
    death_count: usize,
    disabled_potion_effects: Vec<String>,
    achievement_spawned_island_types: Vec<String>,
    visited_modes: Vec<String>,
    unlocked_coll_tiers: Vec<String>,
    crafted_generators: Vec<String>,
    fishing_treasure_caught: usize,
    experience: Vec<SkillExperience>,
    glacite_player_data: GlacitePlayerData,
    jacobs_contest: JacobsContest,
    currencies: Vec<Currencies>,
    dungeons: Dungeons,
    events: Events,
    garden_player_data: GardenPlayerData,
    pet_data: PetData,
    accessory_bag_storage: AccessoryBagStorage,
}

#[derive(Deserialize, Debug)]
struct SubProfileData {
    timestamp: Timestamp,
    invited_by: String,
    confirmed: bool,
    rift: Rift,
    player_data: Option<PlayerData>,
}

#[derive(Deserialize, Debug)]
struct SubProfile {
    coop_invitation: SubProfileData,
    cookie_buff_active: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Member {
    player_id: String,
    profile: Vec<SubProfile>,
}
