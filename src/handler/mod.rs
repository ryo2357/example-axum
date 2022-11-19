use axum::Json;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize,Debug)]
pub struct PostData {
    time: String,
    product_num: String,
    work_num: String,
    machine_num: String,
    pub lot_num: String,
    line_str: String,
    upper_limit: String,
    lower_limit: String,
    instrumentation_times: String,
    is_condition_change: String,
    change_upper_limit: String,
    change_lower_limit: String,
    first_measured_value: String,
    first_measure_decision: String,
    second_measured_value: String,
    second_measure_decision: String,
    third_measured_value: String,
    third_measure_decision: String,
}

pub async fn insert_data(
  Json(body): Json<PostData>
) -> String {
  info!("post");
  info!("{:?}",body);

  return "ok".to_string();
}

pub async fn insert_data(
    Json(body): Json<PostData>
) -> String {
    info!("post");
    info!("{:?}",body);

    return "ok".to_string();
}

async fn sql_test(
  Json(body): Json<router::PostData>,
  Extension(db): Extension<SqlitePool>
) -> String {
  let insert_res: Result<ReturningId<i32>, Error> = sqlx::query_as(
    // r#"
    // INSERT INTO short_links ("url") VALUES ($1) ON CONFLICT ON CONSTRAINT short_links_url_key DO UPDATE set url = excluded.url returning id
    // "#,
    r#"
with e as (
INSERT INTO short_links ("url") VALUES ($1) ON CONFLICT DO NOTHING returning id
) select COALESCE(
(select id from e),
(SELECT id FROM short_links where "url" = $1)
) as id
"#,
  )
  .bind(url)
  .fetch_one(pool)
  .await;
  insert_res.map(|r| r.id)


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

