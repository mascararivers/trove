pub mod enigma;
pub mod locations;
pub mod slayer;

use serde::Deserialize;

use crate::api::skyblock::rift::{
    enigma::Enigma,
    locations::{
        access::Access, blacklagoon::BlackLagoon, castle::Castle, deadcats::DeadCats,
        dreadfarm::Dreadfarm, gallery::Gallery, villageplaza::VillagePlaza,
        westvillage::WestVillage, withercage::WitherCage, wizardtower::WizardTower,
        wyldwoods::WyldWoods,
    },
    slayer::SlayerQuest,
};

#[derive(Debug, Deserialize)]
pub struct InventoryContents {
    #[serde(rename = "type")]
    item_type: usize,
    data: String, // Base64 layered onto NBT or something, i dont really know
}

#[derive(Debug, Deserialize)]
pub struct Inventory {
    inv_contents: Vec<InventoryContents>,
    inv_armor: Vec<InventoryContents>,
    ender_chest_contents: Vec<InventoryContents>,
    ender_chest_page_icons: Vec<Option<String>>,
    equipment_contents: Vec<InventoryContents>,
}

#[derive(Debug, Deserialize)]
pub struct Rift {
    village_plaza: VillagePlaza,
    wither_cage: WitherCage,
    black_lagoon: BlackLagoon,
    dead_cats: DeadCats,
    wizard_tower: WizardTower,
    enigma: Enigma,
    gallery: Gallery,
    slayer_quest: SlayerQuest,
    lifetime_purchased_boundaries: Vec<String>,
    west_village: WestVillage,
    wyld_woods: WyldWoods,
    castle: Castle,
    access: Access,
    dreadfarm: Dreadfarm,
    inventory: Inventory,
}
