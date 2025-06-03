use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Murder {
    step_index: usize,
    room_clues: Vec<String>,
    step_index_pt2: usize,
    step_index_pt3: usize,
}


#[derive(Debug, Deserialize)]
pub struct BarryCenter {
    first_talk_to_barry: bool,
    convinced: Vec<String>,
    received_reward: bool,
}

#[derive(Debug, Deserialize)]
pub struct Cowboy {
    stage: usize,
    hay_eaten: usize,
    rabbit_name: String,
    exported_carrots: usize,
}

#[derive(Debug, Deserialize)]
pub struct BarterBank;

#[derive(Debug, Deserialize)]
pub struct Lonely {
    seconds_sitting: usize
}

#[derive(Debug, Deserialize)]
pub struct Seraphine {
    step_index: usize
}

#[derive(Debug, Deserialize)]
pub struct VillagePlaza {
    murder: Murder,
    barry_center: BarryCenter,
    cowboy: Cowboy,
    barter_bank: BarterBank,
    lonely: Lonely,
    seraphine: Seraphine,
    got_scammed: bool,
}