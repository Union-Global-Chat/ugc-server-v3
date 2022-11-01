use axum::{
    routing::get,
    Router,
    extract::Extension
};

use std::result::Result;
use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    let pool = MySqlPool::connect("")
        .await?;
    let app = Router::new()
        .route("/", get(|| async {"hello"}))
        .layer(Extension(pool));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}