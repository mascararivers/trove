use data::{commission_data::CommissionData, composter_data::ComposterData};
use serde::Deserialize;
pub mod data;

#[derive(Deserialize, Debug)]
pub struct Garden {
    uuid: String,
    commission_data: CommissionData,
    composter_data: ComposterData,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    success: bool,
    garden: Garden,
}

#[derive(Deserialize, Debug)]
pub struct GardenPlayerData {
    copper: usize,
    larva_consumed: usize,
}