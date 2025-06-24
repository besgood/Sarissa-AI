use config::{Config, File, Environment};
use dotenv::dotenv;
use crate::config::modfile::AppConfig;

pub fn load_config() -> AppConfig {
    dotenv().ok();

    let builder = Config::builder()
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("SARISSA").separator("__"));

    builder.build().unwrap().try_deserialize().unwrap()
}