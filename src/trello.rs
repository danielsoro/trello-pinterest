use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{extract, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListChange {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub card: Card,
    pub board: Board,
    #[serde(rename = "listBefore")]
    pub list_before: ListChange,
    #[serde(rename = "listAfter")]
    pub list_after: ListChange,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub data: Data,
    #[serde(rename = "type")]
    pub action_type: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Root {
    pub action: Action,
}

pub fn trello_routes_v1() -> Router {
    Router::new().route("/trello/webhook", post(webhook_handler))
}

async fn webhook_handler(extract::Json(_root): extract::Json<Root>) -> impl IntoResponse {
    StatusCode::OK
}
