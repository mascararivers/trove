
#[derive(Deserialize, Debug)]
struct SubProfile {
    deletion_notice: Timestamp,
}

#[derive(Deserialize, Debug)]
struct Member {
    player_id: String,
    profile: Vec<SubProfile>,
}