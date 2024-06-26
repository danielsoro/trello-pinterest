use axum::Router;
use tokio::net::TcpListener;
use crate::http::trello_routes_v1;

pub async fn start_server() {
    let app_routes_v1 = Router::new().merge(trello_routes_v1());
    let app_v1 = Router::new().nest("/v1", app_routes_v1);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    match axum::serve(listener, app_v1).await {
        Ok(_) => {
            println!("Application started on port 3000")
        }
        Err(e) => {
            println!("Not able to start application because of {e}")
        }
    };
}
