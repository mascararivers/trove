use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct Session {
    online: bool,
    #[serde(rename = "gameType")]
    game_type: String,
    mode: String,
    map: String,
}

#[derive(Debug, Default, Deserialize)]
struct Request {
    success: bool,
    uuid: String,
    session: Option<Session>,
}
