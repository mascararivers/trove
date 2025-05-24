use super::Timestamp;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct SubProfile {
    deletion_notice: Timestamp,
}

#[derive(Deserialize, Debug)]
pub struct Member {
    player_id: String,
    profile: Vec<SubProfile>,
}
