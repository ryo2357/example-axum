use chrono::{DateTime, Local, Duration};

pub fn get_now_filename() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d_%H%M").to_string()
}

pub fn get_detailed_filename() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y%m%d_%H%M%S_%3f").to_string()
}

pub fn get_now_string() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}

pub fn get_before_string() -> String {
    let now: DateTime<Local> = Local::now();
    let dt = now - Duration::hours(25);
    dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}