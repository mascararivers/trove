use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Castle {
    unlocked_pathway_skip: bool,
    fairy_step: usize,
    grubber_stacks: usize,
}
