use super::{super::messages::GameMessage, Game};
use crate::webhook::response::Button;
use actix::prelude::*;
use rand::prelude::*;
use serde_json::Value as JsonValue;
use std::collections::HashSet;

pub struct GameEdible {
    score: u32,
    data: Vec<String>,
    edible: HashSet<String>,
    last_object: Option<String>,
}

impl GameEdible {
    pub fn new() -> Self {
        Self {
            score: 0,
            data: "๐๐๐ฅจ๐ฅ๐๐๐ฟ๐ญ๐ฅ๐๐๐๐ด๐ฅ๐ก๐๐๐๐ฅ๐ฝ๐งโโ๐งโโ๐ฎ๐๐๐๐๐ฒ๐นโธ"
                .chars()
                .map(|c| c.to_string())
                .collect(),
            edible: "๐๐๐ก๐๐ฎ๐๐๐๐๐ฅจ๐ฅ๐๐๐ฟ๐ญ๐ฅ๐๐๐"
                .chars()
                .map(|c| c.into())
                .collect(),
            last_object: None,
        }
    }

    pub fn get_score_text(&self, object: String) -> JsonValue {
        JsonValue::Array(vec![
            JsonValue::String(format!("ะกัะตั {}", self.score)),
            JsonValue::String(object),
        ])
    }

    pub fn show_menu(&self, object: String) -> JsonValue {
        JsonValue::Object(
            [
                ("text", self.get_score_text(object)),
                (
                    "buttons",
                    JsonValue::Array(vec![
                        Button {
                            title: "ััะตะดะพะฑะฝะพะต".into(),
                            payload: JsonValue::Object(serde_json::Map::new()),
                        }
                        .into(),
                        Button {
                            title: "ะฝะตััะตะดะพะฑะฝะพะต".into(),
                            payload: JsonValue::Object(serde_json::Map::new()),
                        }
                        .into(),
                    ]),
                ),
            ]
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect(),
        )
    }
}

impl Game for GameEdible {}

impl Handler<GameMessage> for GameEdible {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: GameMessage, ctx: &mut Self::Context) -> Self::Result {
        let new_obj = self.data.choose(&mut rand::thread_rng()).unwrap().clone();

        match msg {
            GameMessage::Started => {
                self.score = 0;
                Ok(self.show_menu(new_obj))
            }
            GameMessage::Move(game_move) => {
                let command = game_move
                    .data
                    .as_object()
                    .and_then(|obj| obj.get("command"))
                    .and_then(|command| command.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();

                if command == "ััะตะดะพะฑะฝะพะต" {
                    if !self.last_object.is_some()
                        || self.edible.contains(self.last_object.as_ref().unwrap())
                    {
                        self.score += 1;
                        self.last_object = Some(new_obj.clone());
                        Ok(self.show_menu(new_obj))
                    } else {
                        ctx.stop();

                        self.last_object = Some(new_obj.clone());
                        Ok(self.show_error("ะะตะฟัะฐะฒะธะปัะฝะพ".to_string()))
                    }
                } else if command == "ะฝะตััะตะดะพะฑะฝะพะต" {
                    if !self.last_object.is_some()
                        || !self.edible.contains(self.last_object.as_ref().unwrap())
                    {
                        self.score += 1;
                        self.last_object = Some(new_obj.clone());
                        Ok(self.show_menu(new_obj))
                    } else {
                        ctx.stop();

                        self.last_object = Some(new_obj.clone());
                        Ok(self.show_error("ะะตะฟัะฐะฒะธะปัะฝะพ".to_string()))
                    }
                } else {
                    Ok(self.show_error("ะะฒะตะดะตะฝะฐ ะฝะตะธะทะฒะตััะฝะฐั ะบะพะผะฐะฝะดะฐ".to_string()))
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Actor for GameEdible {
    type Context = Context<Self>;
}
