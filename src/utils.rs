use std::fs;
use reqwest::blocking::Client;

pub fn download_file(url: &str, file_name: &str) {
    let client = Client::new();
    let response = client.get(url).send().expect("Failed to download file");
    fs::write(file_name, response.bytes().expect("Failed to read response")).expect("Failed to write file");
}