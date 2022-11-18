use axum::Json;
use serde::Deserialize;

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
