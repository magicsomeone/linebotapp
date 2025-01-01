#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use line_bot_messaging_api::api::LineApiMessagePushResponse;
use line_bot_messaging_api::message::{LineApiMessagePushRequest, LineMessageImage, LineMessageText};
use line_bot_messaging_api::LineClient;

use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub to: Option<String>,
    pub message: Option<String>,
}

#[debug_handler]
pub async fn index(
    State(_ctx): State<AppContext>,
    Json(_payload): Json<Payload>,
) -> Result<Response> {
    let _client_token: String = String::from("/2yGUAc0oJBowI4zIbF0JkykVuXrCKsFGW7VtnK276kQ+dhdtX1A8fgn1pxX/zZflM+Q2fXhMTI8d8wS+7LpwvcQpxTSYG9SeOHaa8roCWhnpmB8ZPxz/V/cy7rFsad7mQeLzjGUuz5iNTE5dxTx6gdB04t89/1O/w1cDnyilFU=");

    let _client = LineClient::new(&_client_token);
    let _txt_message = LineMessageText::new(&_payload.message.unwrap_or(String::from("empty message")));
    let _img_message = LineMessageImage::new(original_content_url, preview_image_url);
    let mut _messages: Vec<Value> = vec![json!(_txt_message)];
    let _request = LineApiMessagePushRequest {
        to: String::from(&_payload.to.unwrap_or(String::from("U4bc597171ccc6ec6c99753a7d98e11e3"))),
        messages: _messages,
        ..Default::default()
    };
    let _response: LineApiMessagePushResponse = _client.message_send_push(&_request).await.unwrap();
    format::text("ok")
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/callback/").add("/", post(index))
}
