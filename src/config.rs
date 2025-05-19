use log::info;

#[derive(Default, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub notification: Notification,
    pub timer: Timer,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Notification {
    pub show_progress_bar: bool,
    pub minimum_update_delay: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct Timer {
    pub ignore_idle_inhibitors: bool,
    pub idle_timeout: u32,         // Seconds
    pub short_break_timeout: u64,  // Seconds
    pub long_break_timeout: u64,   // Seconds
    pub short_break_duration: u64, // Seconds
    pub long_break_duration: u64,  // Seconds
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            show_progress_bar: true,
            minimum_update_delay: 1,
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            ignore_idle_inhibitors: false,
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

        toml::from_str(&std::fs::read_to_string(&config_file).map_or_else(
            |_| String::new(),
            |content| {
                info!("Read config from: {}", &config_file.to_string_lossy());
                content
            },
        ))
        .expect("Failed to parse conifg file")
    }

    fn get_config_file() -> std::path::PathBuf {
        xdg::BaseDirectories::with_prefix(crate::APP_ID)
            .get_config_file("config.toml")
            .expect("Can't find XDG base config directory")
    }
}
