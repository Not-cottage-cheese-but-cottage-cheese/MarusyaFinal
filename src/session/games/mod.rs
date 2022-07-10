use serde_json::Value as JsonValue;

mod edible;
mod game21;

pub use edible::GameEdible;
pub use game21::Game21;

#[derive(Clone)]
pub struct GameMove {
    pub session_id: String,
    pub data: JsonValue,
}

pub trait Game {
    fn show_error(&self, error_text: String) -> JsonValue {
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

    fn end_game(&self, end_text: Vec<String>, speech: Option<String>) -> JsonValue {
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
