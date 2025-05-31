use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct JadeCrystal;

#[derive(Deserialize, Debug)]
pub struct AmberCrystal;
#[derive(Deserialize, Debug)]
pub struct AmethystCrystal;
#[derive(Deserialize, Debug)]
pub struct SapphireCrystal;
#[derive(Deserialize, Debug)]
pub struct TopazCrystal;

#[derive(Deserialize, Debug)]
pub struct Crystals {
    jade_crystal: Option<JadeCrystal>, 
    amber_crystal: Option<AmberCrystal>, 
    amethyst_crystal: Option<AmethystCrystal>, 
    sapphire_crystal: Option<SapphireCrystal>,  
    topaz_crystal: Option<TopazCrystal>, 
}

#[derive(Deserialize, Debug)]
struct Node;

#[derive(Deserialize, Debug)]
struct MiningCore {
    nodes: Vec<Node>,
    tokens_spent: usize,
    hotm_migrator_tree_reset_send_message: bool,
    crystals: Vec<Crystals>,
}