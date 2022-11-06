use std::fs;
use serde_json;

pub struct Config {
    pub owner: String,
    pub repo: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let data = fs::read_to_string("config.json").expect("Unable to read config file");
        let json: serde_json::Value = serde_json::from_str(&data).unwrap();
        let owner = json["owner"].as_str().unwrap();
        let repo = json["repo"].as_str().unwrap();
        Ok(Config {
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }
}
