

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