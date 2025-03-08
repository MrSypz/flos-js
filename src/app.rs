use axum::{routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};
use crate::receive::health::health_check;
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
        .layer(cors)
        .with_state(state);

    Ok(app)
}
