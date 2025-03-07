use axum::{extract::State, Json, http::StatusCode};
use crate::{state::AppState, send::item::Item};
use crate::storage::items::{fetch_items, insert_item};

pub async fn get_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let items = fetch_items(&state.db).await.unwrap();
    Json(items)
}

pub async fn create_item(State(state): State<AppState>, Json(item): Json<Item>) -> (StatusCode, Json<Item>) {
    match insert_item(&state.db, &item).await {
        Ok(_) => (StatusCode::CREATED, Json(item)), // Send back the original item
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Item::default())),
    }
}
