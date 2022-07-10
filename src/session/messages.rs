use super::games::GameMove;
use actix::prelude::{Message, Recipient};
use serde_json::Value as JsonValue;

#[derive(Message, Clone)]
#[rtype(result = "Result<JsonValue, String>")]
pub enum GameMessage {
    Started,
    Move(GameMove),
    Stop(String),
}

#[derive(Message)]
#[rtype(result = "Result<JsonValue, String>")]
pub struct Subscribe {
    pub session_id: String,
    pub recipient: Recipient<GameMessage>,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct IsSubscribed(pub String);
