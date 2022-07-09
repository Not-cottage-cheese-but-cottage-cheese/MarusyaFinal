use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    response: ResponseMeta,
    session: Session,
    version: String,
}

pub struct ResponseBuilder {
    response_meta: ResponseMeta,
    session: Option<Session>,
    version: String,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response_meta: ResponseMeta {
                text: ResponseMetaText::Array(vec![]),
                tts: String::new(),
                buttons: vec![],
                end_session: false,
                card: None,
                commands: vec![],
            },
            session: None,
            version: String::new(),
        }
    }

    pub fn set_session(mut self, session_id: String, user_id: String, message_id: i32) -> Self {
        self.session = Some(Session {
            session_id,
            user_id,
            message_id,
        });

        self
    }

    pub fn set_version(mut self, version: String) -> Self {
        self.version = version;

        self
    }

    pub fn build(self) -> Result<Response, String> {
        if self.session.is_none() {
            return Err("Не указана сессия".into());
        }
        if self.version.is_empty() {
            return Err("Не указана версия".into());
        }

        Ok(Response {
            response: self.response_meta,
            session: self.session.unwrap(),
            version: self.version,
        })
    }
}

#[derive(Serialize)]
pub struct ResponseMeta {
    text: ResponseMetaText,
    tts: String,
    buttons: Vec<Button>,
    end_session: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<serde_json::Value>,
    commands: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct Session {
    session_id: String,
    user_id: String,
    message_id: i32,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ResponseMetaText {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize)]
pub struct Button {
    title: String,
    url: String,
    payload: serde_json::Value, // todo - write struct ButtonPayload
}
