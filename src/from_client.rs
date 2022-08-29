use serde::{Deserialize, Serialize};
use thiserror::Error;
use tungstenite::Message;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FromClient {
    MatchmakingQueue(MatchmakingQueue),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchmakingQueue {
    Register { name: String },
    Leave {},
    HeartbeatCheck {},
}

#[derive(Error, Debug)]
pub enum TryFromErr {
    #[error("error while message to text")]
    Message(#[from] tungstenite::error::Error),
    #[error("error while json parse")]
    Json(#[from] serde_json::error::Error),
}

impl TryFrom<Message> for FromClient {
    type Error = TryFromErr;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let text = value.to_text()?;
        serde_json::from_str::<FromClient>(text).map_err(|e| TryFromErr::Json(e))
    }
}

impl TryFrom<Result<Message, tungstenite::Error>> for FromClient {
    type Error = TryFromErr;

    fn try_from(value: Result<Message, tungstenite::Error>) -> Result<Self, Self::Error> {
        let net = value?;
        let text = net.to_text()?;
        serde_json::from_str::<FromClient>(text).map_err(|e| TryFromErr::Json(e))
    }
}
