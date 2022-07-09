use super::messages::GameMessage;
use actix::prelude::{Actor, Context, Handler};
use serde_json::Value as JsonValue;

pub struct Game21 {}

impl Game21 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler<GameMessage> for Game21 {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: GameMessage, _: &mut Self::Context) -> Self::Result {
        Ok(JsonValue::Null)
    }
}

impl Actor for Game21 {
    type Context = Context<Self>;
}

#[derive(Clone)]
pub struct GameMove {
    pub session_id: String,
    pub data: JsonValue,
}
