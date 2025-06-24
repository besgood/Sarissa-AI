
use once_cell::sync::Lazy;
use config::{Config, File};

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut c = Config::default();
    c.merge(File::with_name("Config")).unwrap_or_else(|_| Config::default());
    c
});
