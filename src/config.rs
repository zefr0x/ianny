extern crate serde;
extern crate toml;
extern crate xdg;

use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_timer")]
    pub timer: Timer,
}

#[derive(Debug, serde::Deserialize)]
pub struct Timer {
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64, // Seconds
    #[serde(default = "default_short_break_timeout")]
    pub short_break_timeout: u64, // Seconds
    #[serde(default = "default_long_break_timeout")]
    pub long_break_timeout: u64, // Seconds
    #[serde(default = "default_short_break_duration")]
    pub short_break_duration: u64, // Seconds
    #[serde(default = "default_long_break_duration")]
    pub long_break_duration: u64, // Seconds
}

fn default_timer() -> Timer {
    Timer {
        idle_timeout: default_idle_timeout(),
        short_break_timeout: default_short_break_timeout(),
        long_break_timeout: default_long_break_timeout(),
        short_break_duration: default_short_break_duration(),
        long_break_duration: default_long_break_duration(),
    }
}

fn default_idle_timeout() -> u64 {
    240 // Seconds (7 minutes)
}

fn default_short_break_timeout() -> u64 {
    1200 // Seconds (20 minutes)
}

fn default_long_break_timeout() -> u64 {
    3840 // Seconds (64 minutes)
}

fn default_short_break_duration() -> u64 {
    120 // Seconds (2 minutes)
}

fn default_long_break_duration() -> u64 {
    240 // Seconds (7 minutes)
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
