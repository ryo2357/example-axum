use chrono::{DateTime, Local};
use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use log4rs::{
    append::console::{ConsoleAppender, Target},
    filter::threshold::ThresholdFilter,
};

const NAME: &'static str = env!("CARGO_PKG_NAME");

pub fn setup_logger() {
    let now: DateTime<Local> = Local::now();
    let log_file_path = "log/".to_string() + &NAME + "_" + &now.format("%Y-%m-%d_%H%M%S").to_string() + ".log";
    make_logger(&log_file_path);
}

// #[cfg(debug_assertions)]
// そもそもでバック用のプログラムなので分岐は市内
fn make_logger(log_file_path: &str) {
    // println!("デバッグ用のロガー");
    // ファイル・コマンドライン
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{l} - {f},{L} - {m}\n")))
        .build();

    // ファイル出力の設定
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})},{d},{f},{L},{M},{m},{n}",
        )))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}
