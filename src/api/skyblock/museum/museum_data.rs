

#[derive(Deserialize, Debug)]
struct Item {
    id: String,
    name: String,
    rarity: String,
    tier: String,
    level: usize,
    amount: usize,
}

#[derive(Deserialize, Debug)]
struct Special {

}

#[derive(Deserialize, Debug)]
struct Profile {
    value: usize,
    items: Vec<Item>,
    appraisal: bool,
    special: Vec<Special>,
}

#[derive(Debug, Desesrialize)]
struct Response {
    success: bool,
    profile: Profile,
}