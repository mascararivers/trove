use data::{commission_data::CommissionData, composter_data::ComposterData};
use serde::Deserialize;
pub mod data;

#[derive(Deserialize, Debug)]
struct Garden {
    uuid: String,
    commission_data: CommissionData,
    composter_data: ComposterData,
}

#[derive(Deserialize, Debug)]
struct Response {
    success: bool,
    garden: Garden,
}
