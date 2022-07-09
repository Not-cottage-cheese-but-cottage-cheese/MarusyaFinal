use serde::Deserialize;
use serde_json::Value as JsonValue;

pub mod response;

#[derive(Deserialize)]
pub struct Request {
    pub meta: JsonValue,
    pub request: JsonValue,
    pub session: Session,
    pub version: String,
}

impl Request {
    pub fn get_nlu(&self) -> Vec<String> {
        self.request
            .as_object()
            .and_then(|req_obj| req_obj.get("nlu"))
            .and_then(|nlu| nlu.as_object())
            .and_then(|nlu| nlu.get("tokens"))
            .and_then(|tokens| tokens.as_array())
            .map(|tokens| {
                tokens
                    .into_iter()
                    .filter_map(|token| token.as_str().map(|s| s.into()))
                    .collect()
            })
            .unwrap_or_default()
    }
}

#[derive(Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub skill_id: String,
    pub new: bool,
    pub message_id: i32,
    pub user: Option<serde_json::Value>,
    pub application: serde_json::Value,
    pub auth_token: serde_json::Value,
}
