use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub database: Database,
    pub category_limits_huf: CategoryLimitsHuf,
}

#[derive(Deserialize)]
pub struct Database {
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryLimitsHuf {
    pub rent: i32,
    pub overhead: i32,
    pub groceries: i32,
    pub health: i32,
    pub sport: i32,
    pub household: i32,
    pub car: i32,
    pub clothes: i32,
    pub restaurant: i32,
    pub courses: i32,
    pub travel: i32,
    pub pension: i32,
    pub rest: i32,
}

pub fn read_config_file() -> Result<Config, Box<dyn Error>> {
    let config_path = "./config.toml";
    let config_contents = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_contents)?;
    Ok(config)
}
