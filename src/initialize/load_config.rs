use confy;
use serde_derive::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(||set_config());

const NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig { 
    pub url: String,
}
// フィールドもpub にしないとアクセスできない

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            url: "127.0.0.1:4000".to_string(),
        }
    }
}
fn set_config() -> AppConfig {
    let path: String = "config/".to_string()+ NAME + ".toml";
    match confy::load_path::<AppConfig>(path){
        Ok(v) => v,
        Err(_e) => AppConfig::default(),
    }
}
