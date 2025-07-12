use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Duels {
    pub best_overall_winstreak: usize,
    pub current_winstreak: usize,
    pub wins: usize,
    pub deaths: usize,
    pub losses: usize,
    pub kills: usize,
}