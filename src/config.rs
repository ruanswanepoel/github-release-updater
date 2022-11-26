use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub owner: String,
    pub repo: String,
}

impl Config {
    pub fn new(file: &str) -> Config {
        let data = fs::read_to_string(file).expect("Unable to read config file");
        serde_json::from_str(&data).expect("Unable to parse config file")
    }

    pub fn init(file: &str) {
        fs::write(file, EXAMPLE_CONTENT).expect("Unable to create config file");
    }
}

const EXAMPLE_CONTENT: &str = r#"{
    "owner": "ruanswanepoel",
    "repo": "host-file-manager",
    "path": "./Host File Manager"
}"#;
