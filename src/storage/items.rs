use crate::storage::database::Database;
use crate::send::item::Item;
use sqlx::{query_as, query};
use sqlx::Error;

pub async fn fetch_items(db: &Database) -> Result<Vec<Item>, Error> {
    let items = query_as!(
        Item,
        "SELECT id, name, description FROM items"
    )
    .fetch_all(&db.pool)
    .await?;
    Ok(items)
}

pub async fn insert_item(db: &Database, item: &Item) -> Result<(), Error> {
    query!(
        "INSERT INTO items (name, description) VALUES ($1, $2) RETURNING id",
        item.name,
        item.description
    )
    .execute(&db.pool)
    .await?;
    Ok(())
}