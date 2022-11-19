#[macro_use]
extern crate log;

use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, StatusCode};
use axum::routing::get;
use axum::routing::post;
use axum::{Extension, Json, Router, Server};
use axum::response::IntoResponse;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;


mod initialize;
use initialize::CONFIG as CONFIG;
mod handler;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize::init();

    // データベースの接続、できない場合エラー
    // データベースの初期化も検討
    let db_path = CONFIG.DATABASE_PATH;

    let options = SqliteConnectOptions::new()
      .filename(db_path)
      .create_if_missing(true);

    let pool = SqlitePool::connect_with(options)
        .await?;


    let app = Router::new()
        .route("/", post(handler::health_check))
        .route("/post_test", post(handler::post_test))
        .route("/sql_test", post(handler::sql_test))
        .layer(Extension(pool));

    info!("Server start.");

    // hyper::Serverの再公開
    Server::bind(&CONFIG.url.parse().unwrap())
       .serve(app.into_make_service())
       .await?;

    Ok(())

}
