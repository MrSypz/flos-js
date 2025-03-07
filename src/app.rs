use axum::{routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use crate::receive::{health::health_check, items::{get_items, create_item}};
use crate::storage::database::Database;
use crate::state::AppState;

pub async fn create_app(database_url: String) -> Result<Router, sqlx::Error> {
    let db = Database::connect(&database_url).await?;
    let state = AppState::new(db);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/items", get(get_items))
        .route("/api/items", post(create_item))
        .layer(cors)
        .with_state(state);

    Ok(app)
}
