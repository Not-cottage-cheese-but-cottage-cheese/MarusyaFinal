use serde_json::Value as JsonValue;

mod game21;
mod edible;

pub use game21::Game21;
pub use edible::Edible;

#[derive(Clone)]
pub struct GameMove {
    pub session_id: String,
    pub data: JsonValue,
}