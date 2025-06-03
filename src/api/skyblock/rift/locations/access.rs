use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Access {
    last_free: usize,
    consumed_prism: bool,
    charge_track_timestamp: usize,
}
