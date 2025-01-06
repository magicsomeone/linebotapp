#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Multipart;
use line_bot_messaging_api::api::LineApiMessagePushResponse;
use line_bot_messaging_api::message::{
    LineApiMessagePushRequest, LineMessageImage, LineMessageText,
};
use line_bot_messaging_api::LineClient;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use tracing::info;

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
    let _client_token: String = String::from("/2yGUAc0oJBowI4zIbF0JkykVuXrCKsFGW7VtnK276kQ+dhdtX1A8fgn1pxX/zZflM+Q2fXhMTI8d8wS+7LpwvcQpxTSYG9SeOHaa8roCWhnpmB8ZPxz/V/cy7rFsad7mQeLzjGUuz5iNTE5dxTx6gdB04t89/1O/w1cDnyilFU=");
    let _client = LineClient::new(&_client_token);
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
    // https://drive.google.com/file/d/16gnnKy_nIMUJJftnvywcg8FFW4RxrBw4/view?usp=drive_link
    let _client_token: String = String::from("/2yGUAc0oJBowI4zIbF0JkykVuXrCKsFGW7VtnK276kQ+dhdtX1A8fgn1pxX/zZflM+Q2fXhMTI8d8wS+7LpwvcQpxTSYG9SeOHaa8roCWhnpmB8ZPxz/V/cy7rFsad7mQeLzjGUuz5iNTE5dxTx6gdB04t89/1O/w1cDnyilFU=");
    let _client = LineClient::new(&_client_token);
    let mut _messages: Vec<Value> = vec![json!(_payload.line_message_image)];
    let _request = LineApiMessagePushRequest {
        to: String::from(&_payload.to),
        messages: _messages,
        ..Default::default()
    };
    let _response: LineApiMessagePushResponse = _client.message_send_push(&_request).await.unwrap();
    format::text("ok")
}

pub async fn image_upload(
    State(_ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.map_err(|err| {
        tracing::error!(error = ?err,"could not read multipart");
        Error::BadRequest("could not read multipart".into())
    })? {
        let file_name = match field.file_name() {
            Some(file_name) => file_name.to_string(),
            _ => return Err(Error::BadRequest("file name not found".into())),
        };

        let content = field.bytes().await.map_err(|err| {
            tracing::error!(error = ?err,"could not readd bytes");
            Error::BadRequest("could not readd bytes".into())
        })?;

        let path = PathBuf::from("/home/yicheng/static").join(file_name);
        let path_str = path.as_path().display();
        _ctx.storage
            .as_ref()
            .upload(path.as_path(), &content)
            .await?;

        file = Some(path.clone());
        info!("path: {}", path_str);
    }

    // file.map_or_else(not_found, |path| {
    //     format::json(views::upload::Response::new(path.as_path()))
    // })
    format::text("ok")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/message/")
        .add("/text/", post(text))
        .add("/image/", post(image))
        .add("/image/upload", post(image_upload))
}
