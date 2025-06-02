use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WitherCage {
    killed_eyes: Vec<String>,
}