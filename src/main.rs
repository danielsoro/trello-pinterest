use trello_pinterest::http::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
