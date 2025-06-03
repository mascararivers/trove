use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Trophy {
    #[serde(rename = "type")]
    trophy_type: String,
    timestamp: usize,
    visits: usize
}

#[derive(Deserialize, Debug)]
pub struct Gallery {
    elise_step: usize,
    secured_trophies: Vec<Trophy>,
    sent_trophy_dialogues: Vec<String>,
}