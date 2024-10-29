use serde::Deserialize;
use std::fs;
use toml::de::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub focus: Colors,
    //pub normal: Colors,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub foreground: u32,
    //pub background: u32,
    pub highlight: u32,
    pub border: u32,
}

pub fn get_theme() -> Result<Config, Error> {
    let config: Config = toml::from_str(&fs::read_to_string("theme.toml").expect(""))
        .expect("Failed to parse theme");

    Ok(config)
}
