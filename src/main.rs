#[macro_use]
extern crate log;

use axum::routing::get;
use axum::routing::post;
use axum::{Extension, Router, Server};

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use sqlx::SqlitePool;


mod initialize;
use initialize::CONFIG as CONFIG;
mod handler;
mod utils;
// mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize::init();

    // データベースの接続、できない場合エラー
    // データベースの初期化も検討
    let db_path = String::new() + &CONFIG.database_path;


    let mut options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);
    // sqlxからのロギングを無視
    options.disable_statement_logging();
      

    let pool = SqlitePool::connect_with(options).await?;

    // フォルダがないError: error returned from database: (code: 14) unable to open database file
    // フォルダはあるがファイルはない⇒からのDBファイルが生成される


    let app = Router::new()
        .route("/", get(handler::health_check))
        .route("/post_test", post(handler::post_test))
        .route("/insert_data", post(handler::insert_data))
        .route("/output_data",get(handler::output_data))
        .route("/output_25h_data",get(handler::output_25h_data))
        .layer(Extension(pool));

    info!("Server start.");

    // hyper::Serverの再公開
    Server::bind(&CONFIG.url.parse().unwrap())
       .serve(app.into_make_service())
       .await?;

    Ok(())

}
