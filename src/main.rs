#[macro_use]
extern crate log;

use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, StatusCode};
use axum::routing::get;
use axum::routing::post;
use axum::{Extension, Json, Router, Server};
use axum::response::IntoResponse;

// use cron::Schedule;
// ジョブスケジューラー

mod initialize;
use initialize::CONFIG as CONFIG;
mod router;
mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize::init();

    // データベースの接続、できない場合スキーマから生成
    let db_path = std::env::var("EXAMPLE_DB").unwrap_or_else(|_| "example.db".to_owned());
    let db = database::connect(&db_path).await?;


    let app = Router::new()
        .route("/", post(router::insert_data))
        .route("/sql_test", post(sql_test))
        .layer(Extension(db.clone()));

    info!("Server start.");

    // hyper::Serverの再公開
    Server::bind(&CONFIG.url.parse().unwrap())
       .serve(app.into_make_service())
       .await?;

    db.close().await;

    Ok(())

}

async fn sql_test(
    Json(body): Json<router::PostData>,
    Extension(db): Extension<database::Db>
) -> String {
    
    info!("lotNo.{}を受信", body.lot_num);

    let insert = "
    INSERT INTO investments (
        name,
        ongoing_charge,
        units,
        avg_unit_cost,
        last_price,
        total_cost,
        value,
        change
    )
    ";

    
    

}

