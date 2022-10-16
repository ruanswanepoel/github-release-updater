use std::fs;

static URL: &'static str = "https://github.com/ruanswanepoel/host-file-manager/releases/latest/download/hfm.exe";

fn main() {
    
    let mut req = reqwest::blocking::get(URL).unwrap();
    let mut file = fs::File::create("hfm.exe").unwrap();
    std::io::copy(&mut req, &mut file).unwrap();

}
