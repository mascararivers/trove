use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlackLagoon {
    talked_to_edwin: bool,
    received_science_paper: bool,
    delivered_science_paper: bool,
    completed_step: usize,
}
