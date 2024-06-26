use axum::http::StatusCode;
use serde_json::json;
use trello_pinterest::server::start_server;
use trello_pinterest::trello::{Action, Board, Card, Data, ListChange, Root};

#[tokio::test]
async fn webhook_should_be_able_to_get_card_move() {
    let _ = tokio::spawn(start_server());

    let hc = httpc_test::new_client("http://localhost:3000").unwrap();
    let response = hc
        .do_post(
            "/v1/trello/webhook",
            json!(Root {
                action: Action {
                    id: "667c8836c75b3e7efe887820".to_string(),
                    data: Data {
                        card: Card {
                            id: "667c82ac89b9d50cb289994c".to_string(),
                            name: "teste".to_string(),
                        },
                        board: Board {
                            id: "667c8175ad220985c5a0d411".to_string(),
                            name: "Development".to_string(),
                        },
                        list_before: ListChange {
                            id: "667c8175ad220985c5a0d418".to_string(),
                            name: "To Do".to_string()
                        },
                        list_after: ListChange {
                            id: "667c8175ad220985c5a0d419".to_string(),
                            name: "Doing".to_string()
                        },
                    },
                    action_type: "updateCard".to_string(),
                    date: "2024-06-26T21:27:22.722Z".to_string(),
                }
            }),
        )
        .await
        .expect("The request to /v1/trello/webhook expected a response but it didn't reply");
    assert_eq!(StatusCode::OK.as_u16(), response.status().as_u16())
}
