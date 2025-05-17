use crate::errors::Result;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Stats {

}

#[derive(Default, Deserialize)]
pub struct Data {
    pub uuid: String,
    pub displayname: String,
    pub rank: String,
    #[serde(rename = "packageRank")] pub package_rank: String,
    #[serde(rename = "newPackageRank")] pub new_package_rank: String,
    #[serde(rename = "monthlyPackageRank")] pub monthly_package_rank: String,
    #[serde(rename="firstLogin")] pub first_login: i64,
    #[serde(rename = "lastLogin")] pub last_login: i64,
    #[serde(rename = "lastLogout")] pub last_logout: i64,
    pub stats: Stats,
}


pub async fn retrieve_player_data(player_id: &str, api_key: String, http_client: reqwest::Client) -> Result<Data> {

    let request = http_client.get(format!("https://api.hypixel.net/v2/player/{}", player_id)).bearer_auth(api_key);
    let response = request.send().await?;

    
    let data: Data = serde_json::from_str(response.text().await?.as_str())?;
    
    Ok(data)
}