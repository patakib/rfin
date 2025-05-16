use crate::budget::init;
use std::error::Error;

mod budget;
mod read_config;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Reading config...");
    let config: read_config::Config = read_config::read_config_file()?;
    init::init_budget(config.category_limits_huf);
    Ok(())
}
