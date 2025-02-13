use std::fs;
use std::process::Command;
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize)]
struct Driver {
    name: String,
    version: String,
    download_url: String,
}

fn main() {
    let drivers = detect_drivers();
    for driver in drivers {
        download_driver(&driver);
    }
    install_drivers();
}

fn detect_drivers() -> Vec<Driver> {
    let mut drivers = Vec::new();
    let output = Command::new("wmic")
        .arg("path")
        .arg("Win32_PnPSignedDriver")
        .arg("get")
        .arg("DeviceID,Name,DriverVersion")
        .output()
        .expect("Failed to execute command");
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let driver = Driver {
                name: parts[1].to_string(),
                version: parts[2].to_string(),
                download_url: format!("https://example.com/drivers/{}.zip", parts[1]),
            };
            drivers.push(driver);
        }
    }
    drivers
}

fn download_driver(driver: &Driver) {
    let client = Client::new();
    let response = client.get(&driver.download_url).send().expect("Failed to download driver");
    let file_path = format!("{}.zip", driver.name);
    fs::write(file_path, response.bytes().expect("Failed to read response")).expect("Failed to write file");
}

fn install_drivers() {
    for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
        if entry.path().extension().map_or(false, |ext| ext == "zip") {
            let _ = Command::new("powershell")
                .arg("-Command")
                .arg(format!("Expand-Archive -Path '{}' -DestinationPath './drivers'", entry.path().display()))
                .output();
        }
    }
}