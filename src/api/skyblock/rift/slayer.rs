use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct SlayerQuest {
    #[serde(rename = "type")]
    quest_type: String,
    tier: usize,
    start_timestamp: usize,
    completion_state: usize,
    used_armor: bool,
    solo: bool,
}