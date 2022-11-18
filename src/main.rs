#[macro_use]
extern crate log;

use axum::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use axum::http::{HeaderValue, StatusCode};
use axum::routing::get;
use axum::routing::post;
use axum::{Extension, Json, Router, Server};

// use cron::Schedule;
// ジョブスケジューラー

mod initialize;
use initialize::CONFIG as CONFIG;
mod router;
mod database;

#[tokio::main]
async fn _main() -> anyhoe::Result<()> {
    initialize::init();

    // データベースの接続、できない場合スキーマから生成
    let db_path = std::env::var("EXAMPLE_DB").unwrap_or_else(|_| "example.db".to_owned());
    // let db = database::connect(&db_path).await?;
    let db = database::connect(&db_path).await?;


    let app = Router::new()
        .route("/", post(router::insert_data))
        // .nest("api",api)
        .route("/sql_test", post(sql_test))
        .layer(Extension(db.clone()));

    info!("Server start.");

    // hyper::Serverの再公開
    Server::bind(&CONFIG.url.parse().unwrap())
       .serve(app.into_make_service())
       .await?
    //    指定されたフューチャが完了したときにサーバーが正常なシャットダウンを処理するように準備します。
    //    .with_graceful_shutdown(async {
    //     tokio::signal::ctrl_c()
    //         .await
    //         .expect("failed to install ctrl+c signal handler")
    //     })
    //    .await
    //    .unwrap_or_else(|_| panic!("Server cannot launch."));
    
    // サーバーが落ちた時、データベースを閉じる
    db.close().await;

    ok(())

}

async fn sql_test(
    Query(query): Query<InsertQuery>,
    Extension(modules): Extension<Arc<Modules>>,
    ) -> Result<impl IntoResponse, impl IntoResponse> {}

// async fn _sql_test(db: Db) -> Result<Json<Vec<Investment>>, StatusCode> {
//     Ok(Json(Investment::all(&db).await.unwrap()))
// }

// // 検証前のコード
// #[tokio::main]
// async fn _main() {
//     initialize::init();

//     let app = Router::new()
//         // .route("/", post(health));
//         .route("/", post(router::insert_data));
//         // .route("/output", get(output));
//         .route("/ADS_test")

//     info!("Server start.");

//     axum::Server::bind(&CONFIG.url.parse().unwrap())
//        .serve(app.into_make_service())
//        .await
//        .unwrap();
// }