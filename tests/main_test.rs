use axum::http::StatusCode;
use trello_pinterest::server::start_server;

#[tokio::test]
async fn should_call_trello_webhook() {
    let _ = tokio::spawn(start_server());

    let hc = httpc_test::new_client("http://localhost:3000").unwrap();
    let response = hc
        .do_get("/v1/trello/webhook")
        .await
        .expect("The request to /v1/trello/webhook expected a response but it didn't reply");
    assert_eq!(StatusCode::OK.as_u16(), response.status().as_u16())
}
