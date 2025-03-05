#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use line_bot_messaging_api::api::LineApiMessagePushResponse;
use line_bot_messaging_api::message::{
    LineApiMessagePushRequest, LineMessageImage, LineMessageText,
};
use line_bot_messaging_api::LineClient;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const CHANNEL_ACCESS_TOKEN: &str = "apuSslKv5D4+2HL3CFzxkxBtJj4VqItj1c4JAKGPy4hwZ9cWnPJLkXXCJQ4Yz/S+zTAoJbnm00Ut5zmkE+VxmSsDYy8/NXvMyKQC7ipLmSwX3zarnXl6GTkHveby29mpKR6bo3W1k+vX5Fl3fDGxPgdB04t89/1O/w1cDnyilFU=";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextPayload {
    pub to: String,
    #[serde(rename = "lineMessageText")]
    pub line_message_text: LineMessageText,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImagePayload {
    pub to: String,
    #[serde(rename = "lineMessageImage")]
    pub line_message_image: LineMessageImage,
}

#[debug_handler]
pub async fn text(
    State(_ctx): State<AppContext>,
    Json(_payload): Json<TextPayload>,
) -> Result<Response> {
    let _client = LineClient::new(CHANNEL_ACCESS_TOKEN);
    let mut _messages: Vec<Value> = vec![json!(_payload.line_message_text)];
    let _request = LineApiMessagePushRequest {
        to: String::from(&_payload.to),
        messages: _messages,
        ..Default::default()
    };
    let _response: LineApiMessagePushResponse = _client.message_send_push(&_request).await.unwrap();
    format::text("ok")
}

#[debug_handler]
pub async fn image(
    State(_ctx): State<AppContext>,
    Json(_payload): Json<ImagePayload>,
) -> Result<Response> {
    let _client = LineClient::new(CHANNEL_ACCESS_TOKEN);
    let mut _messages: Vec<Value> = vec![json!(_payload.line_message_image)];
    let _request = LineApiMessagePushRequest {
        to: String::from(&_payload.to),
        messages: _messages,
        ..Default::default()
    };
    let _response: LineApiMessagePushResponse = _client.message_send_push(&_request).await.unwrap();
    format::text("ok")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/message/")
        .add("/text/", post(text))
        .add("/image/", post(image))
}
