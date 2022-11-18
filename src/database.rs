use std::ops::Deref;

use axum::async_trait;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Db(SqlitePool);

// データベースファイルが場ない場合、schema.sqlからファイルを作製
pub async fn connect(path: &str) -> Result<Db,String> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await
        .unwrap_or_else(|_| panic!("when SqlitePool::connect_with(options)"));

    sqlx::query(include_str!("db/schema.sql"))
        .execute(&pool)
        .await
        .unwrap_or_else(|_| panic!("when sqlx::query(include_str!())"));

    Ok(Db(pool))
}

impl Deref for Db {
    type Target = SqlitePool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<B: Send> FromRequest<B> for Db {
    type Rejection = (StatusCode, &'static str);

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        req.extensions().get().cloned().ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Db not found in request extensions",
        ))
    }
}
