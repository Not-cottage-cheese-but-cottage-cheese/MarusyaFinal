use serde::Serialize;
use serde_json::Value as JsonValue;

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
            response_meta: ResponseMeta::default(),
            session: None,
            version: String::new(),
        }
    }

    pub fn set_response(mut self, response: Result<JsonValue, String>) -> Self {
        self.response_meta = match response {
            Ok(meta) => ResponseMeta::from(meta),
            Err(error_text) => ResponseMeta {
                text: ResponseMetaText::String(error_text),
                end_session: true,
                ..Default::default()
            },
        };

        self
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
    card: Option<JsonValue>,
    commands: Vec<JsonValue>,
}

impl Default for ResponseMeta {
    fn default() -> Self {
        Self {
            text: ResponseMetaText::Array(vec![]),
            tts: String::new(),
            buttons: vec![],
            end_session: false,
            card: None,
            commands: vec![],
        }
    }
}

impl From<JsonValue> for ResponseMeta {
    fn from(value: JsonValue) -> Self {
        let mut result = Self::default();

        if let Some(obj) = value.as_object() {
            result.text = obj
                .get("text")
                .map(|text| ResponseMetaText::from(text.clone()))
                .unwrap_or(ResponseMetaText::Array(vec![]));
            result.tts = obj
                .get("tts")
                .and_then(|s| s.as_str().map(|s| s.into()))
                .unwrap_or("".into());
            result.buttons = obj
                .get("buttons")
                .and_then(|buttons| buttons.as_array())
                .map(|buttons| {
                    buttons
                        .into_iter()
                        .filter_map(|b| Button::try_from(b.clone()).ok())
                        .collect()
                })
                .unwrap_or_default();
            result.end_session = obj
                .get("end_session")
                .and_then(|is_end| is_end.as_bool())
                .unwrap_or_default();
        }

        result
    }
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

impl From<JsonValue> for ResponseMetaText {
    fn from(value: JsonValue) -> Self {
        if let Some(array) = value.as_array() {
            Self::Array(
                array
                    .into_iter()
                    .filter_map(|s| s.as_str().map(|s| s.into()))
                    .collect(),
            )
        } else if let Some(string) = value.as_str() {
            Self::String(string.into())
        } else {
            Self::Array(vec![])
        }
    }
}

#[derive(Serialize)]
pub struct Button {
    pub title: String,
    // url: String,
    pub payload: JsonValue, // todo - write struct ButtonPayload
}

impl TryFrom<JsonValue> for Button {
    type Error = String;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        if let Some(obj) = value.as_object() {
            let title = obj
                .get("title")
                .and_then(|title| title.as_str())
                .map(|title| title.to_string())
                .filter(|title| !title.is_empty());
            // let url = value
            //     .get("url")
            //     .and_then(|url| url.as_str())
            //     .map(|url| url.to_string())
            //     .filter(|url| !url.is_empty());
            let payload = obj.get("payload");

            match title.zip(payload) {
                Some((title, payload)) => Ok(Button {
                    title,
                    // url,
                    payload: payload.to_owned(),
                }),
                None => Err("Передан неправильный объект".into()),
            }
        } else {
            Err("Значение не является объектом".into())
        }
    }
}

impl From<Button> for JsonValue {
    fn from(b: Button) -> Self {
        Self::Object(
            [
                ("title", JsonValue::String(b.title)),
                ("payload", b.payload),
            ]
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
        )
    }
}
