use crate::errors::Result;
use serde::Deserialize;

#[derive(Default, Deserialize, Debug)]
pub struct Player {
    pub uuid: String,
    pub displayname: String,
    pub rank: Option<String>,
    #[serde(rename = "packageRank")]
    pub package_rank: Option<String>,
    #[serde(rename = "newPackageRank")]
    pub new_package_rank: Option<String>,
    #[serde(rename = "monthlyPackageRank")]
    pub monthly_package_rank: Option<String>,
    #[serde(rename = "firstLogin")]
    pub first_login: i64,
    #[serde(rename = "lastLogin")]
    pub last_login: i64,
    #[serde(rename = "lastLogout")]
    pub last_logout: i64,
}

#[derive(Default, Deserialize, Debug)]
pub struct Data {
    success: bool,
    player: Player,
}

pub const API_URL: &str = "https://api.hypixel.net/v2/";

pub async fn retrieve_player_data(
    player_id: String,
    api_key: String,
    http_client: reqwest::Client,
) -> Result<Data> {
    let request = http_client
        .get(format!("{}player?uuid={}", API_URL, player_id))
        .header("API-Key", api_key);
    let response = request.send().await?;

    let text = response.text().await?;

    println!("{:#?}", text);

    let data: Data = serde_json::from_str::<Data>(&text)?;

    Ok(data)
}
