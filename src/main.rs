mod config;

use std::fs;
use config::Config;
use std::{thread, time::Duration};

const CONFIG_FILE: &str = "config.json";
const VERSION_FILE: &str = "version";

fn main() {

    if !fs::metadata(CONFIG_FILE).is_ok() {
        Config::init(CONFIG_FILE);
    }

    if !fs::metadata(VERSION_FILE).is_ok() {
        fs::write(VERSION_FILE, "0.0.0").expect("Failed to create version file");
    }

    let config = Config::new(CONFIG_FILE);
    let base_url = format!("https://github.com/{}/{}", config.owner, config.repo);
    let version_url = format!("{}/releases/latest", base_url);
    let download_url = format!("{}/releases/latest/download/hfm.exe", base_url);

    loop {
        
        let current = get_current_version();
        let latest = get_version_from_url(&version_url);
    
        if current != latest {
            println!("Found a newer version of {} (Current = {}, Latest = {}). Downloading...", config.repo, current, latest);
            download_from_url(&download_url, &config.filename);
            set_current_version(latest);
        }

        thread::sleep(Duration::from_secs(10));

    }

}

fn get_current_version() -> String {
    fs::read_to_string(VERSION_FILE).expect("Failed to read version file")
}

fn set_current_version(version: String) {
    fs::write(VERSION_FILE, version).expect("Failed to write to version");
}

/// Gets the latest version from the given url and returns it as a string
fn get_version_from_url(url: &String) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let body = response.text().expect("Failed to read response body");
    let json: serde_json::Value = serde_json::from_str(&body).expect("Failed to parse JSON");
    let tag_name = json["tag_name"].as_str().expect("Failed to get tag_name from response json");
    tag_name.to_string()
}

/// Downloads the file from the given url and saves it to the current directory
fn download_from_url(url: &String, filename: &String) {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let mut file = fs::File::create(filename).expect("Failed to create file");
    std::io::copy(&mut response, &mut file).expect("Failed to copy contents from response to file");
}
