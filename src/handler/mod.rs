use axum::{Json,Extension};
use serde::Deserialize;
use sqlx::SqlitePool;
use sqlx::query;
use sqlx::Row;

#[derive(Deserialize,Debug)]
pub struct PostData {
    time: String,
    product_num: i32,
    work_num: i32,
    machine_num: i32,
    lot_num: i32,
    line_str: String,
    upper_limit: i32,
    lower_limit: i32,
    instrumentation_times: i32,
    is_condition_change: bool,
    // SQLiteにboolはないのでTEXTのTRUE,FALSEで保存
    change_upper_limit: i32,
    change_lower_limit: i32,
    first_measured_value: i32,
    first_measure_decision: bool,
    second_measured_value: i32,
    second_measure_decision: bool,
    third_measured_value: i32,
    third_measure_decision: bool,
}

pub async fn health_check() -> String {
    info!("API:health");
    "Hello, developer.".to_string()
}

pub async fn post_test(body: String) -> String {
    info!("API:post_test");
    info!("{:?}",body );
    return "ok".to_string();
}

pub async fn insert_data(
  Json(body): Json<PostData>,
  Extension(db): Extension<SqlitePool>
) -> String {
    info!("API:insert_data");
    let sql_result = query(r#"
        insert into investments(
            time, product_num, work_num, machine_num, lot_num, line_str, upper_limit, lower_limit, instrumentation_times, is_condition_change, change_upper_limit, change_lower_limit, first_measured_value, first_measure_decision, second_measured_value, second_measure_decision, third_measured_value, third_measure_decision 
        ) values (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#)
        .bind(body.time)
        .bind(body.product_num)
        .bind(body.work_num)
        .bind(body.machine_num)
        .bind(body.lot_num)
        .bind(body.line_str)
        .bind(body.upper_limit)
        .bind(body.lower_limit)
        .bind(body.instrumentation_times)
        .bind(bool_to_string(body.is_condition_change))
        .bind(body.change_upper_limit)
        .bind(body.change_lower_limit)
        .bind(body.first_measured_value)
        .bind(bool_to_string(body.first_measure_decision))
        .bind(body.second_measured_value)
        .bind(bool_to_string(body.second_measure_decision))
        .bind(body.third_measured_value)
        .bind(bool_to_string(body.third_measure_decision))
        .execute(&db)
        .await;
    
    match sql_result {
        Ok(t) => info!("SQL挿入成功:{:?}",t),
        Err(_) => {
            info!("SQL挿入失敗");
            return "ng".to_string();
        }
    }

    return "ok".to_string()
}

fn bool_to_string(bool:bool) -> String {
    if bool {
        return "True".to_string()
    }
    return "FALSE".to_string()
}

pub async fn output_data(Extension(db): Extension<SqlitePool>) -> String {
    info!("API:output_data");
    
    let end = "2022-11-21 15:09:00.000".to_string();
    let start = "2022-11-21 15:07:00.000".to_string();


    let sql_result = query(r#"
        SELECT * FROM investments WHERE time >= ? AND time <= ?
        "#)
        .bind(start)
        .bind(end)
        .fetch_all(&db)
        .await;

    let rows = match sql_result {
        Ok(t) => {
            info!("SQL検索成功");
            t
        },
        Err(t) => {
            info!("SQL検索失敗:{:?}",t);
            return "NG".to_string();
        }
    };

    let result = rows
        .iter()
        .map(|r| format!("{},{}", r.get::<i64, _>("id"), r.get::<String, _>("time")))
        .collect::<Vec<String>>();
    
    info!("{:?}",result);

    // fileクレートの出力メソッド（result）
    // 出力可否のハンドリング
    

    return "OK".to_string();

}

    
