#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use line_bot_messaging_api::api::LineApiMessagePushResponse;
use line_bot_messaging_api::message::{LineApiMessagePushRequest, LineMessageText};
use line_bot_messaging_api::LineClient;
use line_bot_messaging_api::webhook::LineWebhook;
use loco_rs::prelude::*;
use serde_json::{json, Value};
use tracing::log;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    // LineClient::web
    let _client = LineClient::new("/2yGUAc0oJBowI4zIbF0JkykVuXrCKsFGW7VtnK276kQ+dhdtX1A8fgn1pxX/zZflM+Q2fXhMTI8d8wS+7LpwvcQpxTSYG9SeOHaa8roCWhnpmB8ZPxz/V/cy7rFsad7mQeLzjGUuz5iNTE5dxTx6gdB04t89/1O/w1cDnyilFU=");
    let _message = LineMessageText::new("Hello, World!");
    let mut _messages: Vec<Value> = vec![json!(_message)];
    let _request = LineApiMessagePushRequest {
        to: String::from("U4bc597171ccc6ec6c99753a7d98e11e3"),
        messages: _messages,
        ..Default::default()
    };
    let _response: LineApiMessagePushResponse = _client.message_send_push(&_request).await.unwrap();
    format::text("ok")
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/linebots/").add("/", get(index))
}
