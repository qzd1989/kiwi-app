use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

pub enum Msg {
    Error { message: String },
    Ok { data: Value },
}

impl Msg {
    fn to_message(&self) -> Message {
        match self {
            Msg::Error { message } => Message::Text(
                json!({
                    "status": "error",
                    "message": message,
                    "data": null
                })
                .to_string()
                .into(),
            ),
            Msg::Ok { data } => Message::Text(
                json!({
                    "status": "success",
                    "message": "operation successful",
                    "data": data
                })
                .to_string()
                .into(),
            ),
        }
    }
}

impl Msg {
    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::Error {
            message: message.into(),
        }
    }

    pub fn ok(data: Value) -> Self {
        Self::Ok { data }
    }
}

impl From<Msg> for Message {
    fn from(msg: Msg) -> Self {
        msg.to_message()
    }
}
