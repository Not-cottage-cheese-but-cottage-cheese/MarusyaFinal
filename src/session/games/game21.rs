use super::super::messages::GameMessage;
use crate::webhook::response::Button;
use actix::prelude::*;
use rand::prelude::*;
use serde_json::Value as JsonValue;

#[derive(Default)]
pub struct Game21 {
    score: i32,
    deck: Vec<i32>,
}

impl Game21 {
    pub fn new() -> Self {
        Self {
            score: 0,
            deck: vec![2, 3, 4, 6, 7, 8, 9, 10, 11],
        }
    }

    pub fn get_score_text(&self, value: i32) -> JsonValue {
        JsonValue::Array(vec![
            JsonValue::String(format!("Счет {}", self.score)),
            JsonValue::String(value.to_string()),
        ])
    }

    pub fn show_menu(&self, value: i32) -> JsonValue {
        JsonValue::Object(
            [
                ("text", self.get_score_text(value)),
                (
                    "tts",
                    JsonValue::String(format!("Тек`ущий сч`ёт `равен {}", self.score)),
                ),
                (
                    "buttons",
                    JsonValue::Array(vec![
                        Button {
                            title: "еще".into(),
                            payload: JsonValue::Object(
                                [("action", JsonValue::String("more".into()))]
                                    .into_iter()
                                    .map(|(key, value)| (key.to_string(), value))
                                    .collect(),
                            ),
                        }
                        .into(),
                        Button {
                            title: "хватит".into(),
                            payload: JsonValue::Object(
                                [("action", JsonValue::String("enough".into()))]
                                    .into_iter()
                                    .map(|(key, value)| (key.to_string(), value))
                                    .collect(),
                            ),
                        }
                        .into(),
                    ]),
                ),
            ]
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
        )
    }

    pub fn show_error(&self, error_text: String) -> JsonValue {
        JsonValue::Object(
            [
                ("text", JsonValue::String(error_text)),
                ("tts", JsonValue::String(format!("Игр`а закончен`а"))),
                ("end_session", JsonValue::Bool(true)),
            ]
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
        )
    }

    pub fn end_game(&self, end_text: Vec<String>, speech: Option<String>) -> JsonValue {
        JsonValue::Object(
            [
                (
                    "text",
                    JsonValue::Array(
                        end_text
                            .into_iter()
                            .map(|text| JsonValue::String(text))
                            .collect(),
                    ),
                ),
                (
                    "tts",
                    speech
                        .map(|t| JsonValue::String(t))
                        .unwrap_or(JsonValue::String(format!("Игр`а зак`ончена"))),
                ),
                ("end_session", JsonValue::Bool(true)),
            ]
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
        )
    }
}

impl Handler<GameMessage> for Game21 {
    type Result = Result<JsonValue, String>;

    fn handle(&mut self, msg: GameMessage, ctx: &mut Self::Context) -> Self::Result {
        let new_card = *self.deck.choose(&mut rand::thread_rng()).unwrap();

        match msg {
            GameMessage::Started => {
                self.score = new_card;
                Ok(self.show_menu(new_card))
            }
            GameMessage::Move(game_move) => {
                let command = game_move
                    .data
                    .as_object()
                    .and_then(|obj| obj.get("command"))
                    .and_then(|command| command.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();

                if command == "еще" {
                    self.score += new_card;
                    if self.score > 21 {
                        ctx.stop();

                        Ok(self.end_game(
                            vec![format!("Счет {}", self.score), "Вы проиграли".to_string()],
                            Some("Вы проиграли".to_string()),
                        ))
                    } else if self.score == 21 {
                        ctx.stop();

                        Ok(self.end_game(
                            vec![format!("Счет {}", self.score), "Победа!".to_string()],
                            Some("Вы побед`или".to_string()),
                        ))
                    } else {
                        Ok(self.show_menu(new_card))
                    }
                } else if command == "хватит" || command == "on_interrupt" {
                    ctx.stop();
                    Ok(self.end_game(vec!["Игра закончена".to_string()], None))
                } else {
                    Ok(self.show_error("Введена неизвестная команда".to_string()))
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Actor for Game21 {
    type Context = Context<Self>;
}
