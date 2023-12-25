use std::process::Command;
use std::env;

fn main() {
    let output = Command::new("pbpaste")
        .output()
        .expect("Failed to execute command");

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if url.is_empty() {
        println!("No URL provided. Please copy a URL to your clipboard.");
        return;
    }

    let home_dir = env::var("HOME").unwrap_or(".".to_string());
    let download_dir = format!("{}/Downloads", home_dir);

    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg(&url)
        .current_dir(download_dir)
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Download completed successfully.");
    } else {
        println!("Download failed.");
    }
}