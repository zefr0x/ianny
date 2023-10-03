extern crate serde;
extern crate toml;
extern crate xdg;

use std::path::PathBuf;

#[derive(Default, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub timer: Timer,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Timer {
    pub idle_timeout: u64,         // Seconds
    pub short_break_timeout: u64,  // Seconds
    pub long_break_timeout: u64,   // Seconds
    pub short_break_duration: u64, // Seconds
    pub long_break_duration: u64,  // Seconds
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            idle_timeout: 240,         // Seconds (7 minutes)
            short_break_timeout: 1200, // Seconds (20 minutes)
            long_break_timeout: 3840,  // Seconds (64 minutes)
            short_break_duration: 120, // Seconds (2 minutes)
            long_break_duration: 240,  // Seconds (7 minutes)
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_file = Self::get_config_file();

        toml::from_str(&match std::fs::read_to_string(&config_file) {
            Ok(content) => {
                eprintln!("Read config from: {}", &config_file.to_string_lossy());

                content
            }
            Err(_) => String::new(),
        })
        .expect("Failed to parse conifg file")
    }

    fn get_config_file() -> PathBuf {
        xdg::BaseDirectories::with_prefix(crate::APP_ID)
            .unwrap()
            .get_config_file("config.toml")
    }
}
