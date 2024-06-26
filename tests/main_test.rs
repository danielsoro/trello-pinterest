use axum::http::StatusCode;
use trello_pinterest::server::start_server;

#[tokio::test]
async fn webhook_should_be_able_to_get_card_move() {
    let _ = tokio::spawn(start_server());

    let hc = httpc_test::new_client("http://localhost:3000").unwrap();
    let response = hc
        .do_post(
            "/v1/trello/webhook",
            (
                r#"{
               "action":{
                  "id":"667c87ba8342b74232b7bab5",
                  "data":{
                     "card":{
                        "id":"667c82ac89b9d50cb289994c",
                        "name":"teste"
                     },
                     "board":{
                        "id":"667c8175ad220985c5a0d411",
                        "name":"Development"
                     },
                     "listBefore":{
                        "id":"667c8175ad220985c5a0d418",
                        "name":"To Do"
                     },
                     "listAfter":{
                        "id":"667c8175ad220985c5a0d419",
                        "name":"Doing"
                     }
                  },
                  "type":"updateCard",
                  "date":"2024-06-26T21:27:22.722Z"
               }
            }"#,
                "application/json",
            ),
        )
        .await
        .expect("The request to /v1/trello/webhook expected a response but it didn't reply");
    assert_eq!(StatusCode::OK.as_u16(), response.status().as_u16())
}
