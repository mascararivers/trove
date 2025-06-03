use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contest {
    collected: usize,
    claimed_rewards: Option<bool>,
    claimed_position: Option<usize>,
    claimed_participants: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct Medal {
    bronze: usize,
    silver: usize,
    gold: usize,
}

#[derive(Debug, Deserialize)]
pub struct Perk;

#[derive(Debug, Deserialize)]
pub struct JacobsContest {
    medals_inv: Vec<Medal>,
    perks: Vec<Perk>,
    contests: Vec<Contest>,
    talked: bool,
}
