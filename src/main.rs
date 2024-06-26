use trello_pinterest::server::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
