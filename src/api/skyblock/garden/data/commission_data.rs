use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Visits {
    jerry: usize,
    jacob: usize,
    andrew: usize,
}

#[derive(Deserialize, Debug)]
struct Completed {
    jerry: usize,
    jacob: usize,
    andrew: usize,
}

#[derive(Deserialize, Debug)]
pub struct CommissionData {
    visits: Visits,
    completed: Completed,
    total_completed: usize,
    unique_npcs_served: usize,
}
