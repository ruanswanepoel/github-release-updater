use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub owner: String,
    pub repo: String,
    pub filename: String,
}

impl Config {
    pub fn new(file: &str) -> Config {
        let data = fs::read_to_string(file).expect("Failed to read config file");
        serde_json::from_str(&data).expect("Failed to parse config file")
    }

    pub fn init(file: &str) {
        fs::write(file, EXAMPLE_CONTENT).expect("Failed to create config file");
    }
}

const EXAMPLE_CONTENT: &str = r#"{
    "owner": "ruanswanepoel",
    "repo": "host-file-manager",
    "filename": "hfm.exe"
}"#;
