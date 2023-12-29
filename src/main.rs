use axum::Server;
use std::net::SocketAddr;

use todo_backend::api::routes::app_router;

#[tokio::main]
async fn main() {
    let app = app_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
