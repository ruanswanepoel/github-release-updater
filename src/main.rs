mod config;

use std::fs;
use config::Config;

const CONFIG_FILE: &str = "config.json";
const VERSION_FILE: &str = "version";

fn main() {

    if !fs::metadata(CONFIG_FILE).is_ok() {
        Config::init(CONFIG_FILE);
    }

    let config = Config::new(CONFIG_FILE);
    let base_url = format!("https://github.com/{}/{}", config.owner, config.repo);
    let version_url = format!("{}/releases/latest", base_url);
    let download_url = format!("{}/releases/latest/download/hfm.exe", base_url);

    if !fs::metadata(VERSION_FILE).is_ok() {
        fs::write(VERSION_FILE, "0.0.0").expect("Unable to create version file");
    }
    
    let current = get_current_version();
    let latest = get_version_from_url(&version_url);

    if current != latest {
        println!("Download latest version of {} (Current = {}, Latest = {})", config.repo, current, latest);
        download_from_url(&download_url);
        set_current_version(latest);
        println!("Done!");
    } else {
        println!("No new version available");
    }

}

fn get_current_version() -> String {
    fs::read_to_string(VERSION_FILE).expect("Unable to read version file")
}

fn set_current_version(version: String) {
    fs::write(VERSION_FILE, version).expect("Unable to write to version");
}

fn get_version_from_url(url: &String) -> String {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let body = response.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let tag_name = json["tag_name"].as_str().unwrap();
    tag_name.to_string()
}

fn download_from_url(url: &String) {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let mut file = fs::File::create("hfm.exe").unwrap();
    std::io::copy(&mut response, &mut file).unwrap();
}
