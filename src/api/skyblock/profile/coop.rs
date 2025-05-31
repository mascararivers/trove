use serde::{Deserialize};
use crate::api::skyblock::{dungeons::Dungeons, garden::data::jacobs_contest::{self, JacobsContest}};
use super::{banking::Currencies, Timestamp};

#[derive(Deserialize, Debug)]  
struct Perk {
    catacombs_boss_luck: usize,
    catacombs_looting: usize,
    catacombs_strength: usize,
}

#[derive(Deserialize, Debug)]
struct PotionEffect;

#[derive(Deserialize, Debug)]
struct PlayerData {
    visited_zones: Vec<String>,
    last_death: Option<Timestamp>,
    perks: Vec<Perk>,
    active_effects: Vec<PotionEffect>,
    paused_efffects: Vec<PotionEffect>,
    death_count: usize,
    disabled_potion_effects: Vec<PotionEffect>,
    visited_modes: Vec<String>,
    unlocked_coll_tiers: Vec<String>,
    crafted_generators: Vec<String>,
    fishing_treasure_caught: usize,
    jacobs_contest: JacobsContest,
    currencies: Vec<Currencies>,
    dungeons: Dungeons,
}

#[derive(Deserialize, Debug)]
struct SubProfileData {
    timestamp: Timestamp,
    invited_by: String,
    confirmed: bool,
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