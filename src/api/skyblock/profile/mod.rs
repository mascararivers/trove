use banking::Banking;
use coop::Member;
use serde::Deserialize;
mod banking;
mod coop;

#[derive(Deserialize, Debug)]
struct Timestamp {
    timestamp: usize,
}

#[derive(Deserialize, Debug)]
struct CommunityUpgrades {
    upgrade_id: String,
    level: usize,
}

#[derive(Deserialize, Debug)]
struct Profile {
    profile_id: String,
    members: Vec<Member>,
    cute_name: String,
    selected: bool,
    community_upgrades: Vec<CommunityUpgrades>,
    banking: Banking,
    game_mode: String,
}

struct Response {
    success: bool,
    profile: Profile,
}
