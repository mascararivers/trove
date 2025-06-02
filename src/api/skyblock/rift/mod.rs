pub mod locations;
use serde::Deserialize;
use crate::api::skyblock::rift::locations::villageplaza::VillagePlaza;



#[derive(Debug, Deserialize)]
pub struct Rift {
    village_plaza: VillagePlaza,
}