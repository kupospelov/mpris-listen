use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub player: PlayerConfig,
    pub commands: CommandConfig,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerConfig {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommandConfig {
    #[serde(default)]
    pub play: String,
    #[serde(default)]
    pub pause: String,
    #[serde(default)]
    pub playpause: String,
    #[serde(default)]
    pub stop: String,
    #[serde(default)]
    pub previous: String,
    #[serde(default)]
    pub next: String,
}

pub fn load_config() -> Config {
    let user_config = match env::var("XDG_CONFIG_HOME") {
        Ok(v) => read_config(&v),
        Err(_) => match env::var("HOME") {
            Ok(p) => read_config(&format!("{}/.config", &p)),
            Err(_) => None,
        },
    };

    match user_config {
        Some(c) => parse_config(c),
        None => match read_config("/etc/mpris-listen/config.toml") {
            Some(c) => parse_config(c),
            None => error!("No configuration file"),
        },
    }
}

fn parse_config(config: String) -> Config {
    match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => error!("Unable to parse the configuration: {}", e),
    }
}

fn read_config(config_dir: &str) -> Option<String> {
    let path = format!("{}/mpris-listen/config.toml", config_dir);
    fs::read_to_string(&path).ok()
}
