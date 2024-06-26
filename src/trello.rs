use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TrelloRequest {}

#[derive(Serialize)]
pub struct TrelloResponse {
    value: String,
}

pub fn trello_routes_v1() -> Router {
    Router::new().route("/trello/webhook", get(webhook_handler))
}

async fn webhook_handler() -> Json<TrelloResponse> {
    Json(TrelloResponse {
        value: "Hello Trello Webhook".to_string(),
    })
}
