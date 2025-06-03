use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct Dreadfarm {
    shania_stage: usize,
    caducous_feeder_uses: Vec<usize>
}