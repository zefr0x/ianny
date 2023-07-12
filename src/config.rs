extern crate serde;
extern crate toml;
extern crate xdg;

use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64, // Minutes
    #[serde(default = "default_short_break_timeout")]
    pub short_break_timeout: u64, // Minutes
    #[serde(default = "default_long_break_tiemout")]
    pub long_break_tiemout: u64, // Minutes
    #[serde(default = "default_short_break_duration")]
    pub short_break_duration: u64, // Minutes
    #[serde(default = "default_long_break_duration")]
    pub long_break_duration: u64, // Minutes
}

fn default_idle_timeout() -> u64 {
    7 // Minutes
}

fn default_short_break_timeout() -> u64 {
    20 // Minutes
}

fn default_long_break_tiemout() -> u64 {
    64 // Minutes
}

fn default_short_break_duration() -> u64 {
    2 // Minutes
}

fn default_long_break_duration() -> u64 {
    7 // Minutes
}

pub fn get_config_file() -> PathBuf {
    xdg::BaseDirectories::with_prefix(crate::APP_ID)
        .unwrap()
        .get_config_file("config.toml")
}

pub fn load_config(config_file_path: PathBuf) -> Config {
    toml::from_str(&match std::fs::read_to_string(&config_file_path) {
        Ok(content) => {
            eprintln!("Read config from: {}", &config_file_path.to_string_lossy());

            content
        }
        Err(_) => String::new(),
    })
    .expect("Failed to parse conifg file")
}
