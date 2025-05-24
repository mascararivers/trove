use crate::errors::Result;
use serde::Deserialize;

use super::player_data::API_URL;

#[derive(Debug, Default, Deserialize)]
pub struct Game {
    date: usize,
    #[serde(rename = "gameType")]
    game_type: String,
    mode: String,
    map: String,
    ended: usize,
}

#[derive(Debug, Default, Deserialize)]
pub struct RecentlyPlayed {
    success: bool,
    uuid: String,
    games: Vec<Game>,
}

pub async fn get_recently_played(
    uuid: String,
    api_key: String,
    http_client: reqwest::Client,
) -> Result<RecentlyPlayed> {
    let request = http_client
        .get(format!("{}player?uuid={}", API_URL, uuid))
        .header("API-Key", api_key);
    let response = request.send().await?;

    let text = response.text().await?;

    let data: RecentlyPlayed = serde_json::from_str::<RecentlyPlayed>(&text)?;

    Ok(data)
}
