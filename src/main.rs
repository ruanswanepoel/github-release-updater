use std::fs;

static VERSION_URL: &'static str = "https://github.com/ruanswanepoel/host-file-manager/releases/latest";
static DOWNLOAD_URL: &'static str = "https://github.com/ruanswanepoel/host-file-manager/releases/latest/download/hfm.exe";

fn main() {

    if !fs::metadata("version.txt").is_ok() {
        fs::write("version.txt", "0.0.0").expect("Unable to create version.txt");
    }
    
    let current = get_current_version();
    let latest = get_latest_version();

    if current != latest {
        println!("New version available: {}", latest);
        println!("Downloading...");
        download_latest_version();
        set_current_version(latest);
        println!("Done!");
    } else {
        println!("No new version available");
    }

}

fn get_current_version() -> String {
    fs::read_to_string("version.txt").unwrap()
}

fn set_current_version(version: String) {
    fs::write("version.txt", version).expect("Unable to write to version.txt");
}

fn get_latest_version() -> String {
    let client = reqwest::blocking::Client::new();
    let response = client.get(VERSION_URL)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let body = response.text().unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let tag_name = json["tag_name"].as_str().unwrap();
    tag_name.to_string()
}

fn download_latest_version() {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(DOWNLOAD_URL)
        .header("Accept", "application/json")
        .send()
        .unwrap();
    let mut file = fs::File::create("hfm.exe").unwrap();
    std::io::copy(&mut response, &mut file).unwrap();
}
