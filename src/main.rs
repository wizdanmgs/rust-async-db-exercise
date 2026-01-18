mod db;
mod errors;
mod models;
mod routes;
mod state;

use axum::Router;
use state::AppState;
use tokio::net::TcpListener;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    let pool = db::init_db().await;
    let state = AppState { db: pool };

    let app: Router = routes::router(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
