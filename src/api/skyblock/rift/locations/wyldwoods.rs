use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WyldWoods {
    sirius_started_q_a: bool,
    sirius_q_a_chain_done: bool,
    sirius_completed_q_a: bool,
    bughunter_step: usize,
    talked_threebrothers: Vec<String>,
}
