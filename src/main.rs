mod app;
mod state;
mod receive;
mod send;
mod storage;
mod config;


use crate::app::create_app;
use config::Config;
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let app = create_app(config.database_url.clone()).await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
