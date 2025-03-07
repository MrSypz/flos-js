use sqlx::{Pool, Postgres, PgPool};
use std::env;
use dotenv::dotenv;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect() -> Result<Self, sqlx::Error> {
        dotenv().ok(); // Load .env file

        // PostgreSQL connection string format:
        // postgresql://username:password@localhost:5432/database_name
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in environment variables");

        println!("Connecting to database: {}", database_url); // Debugging

        let pool = PgPool::connect(&database_url).await?;
        
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("../migrations").run(&self.pool).await?;
        Ok(())
    }
}