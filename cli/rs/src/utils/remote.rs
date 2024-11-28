use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::{cmp::min, fmt::Write};
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

use reqwest;

/**
 * Start Download
 * 
 * Will manage a download and display progress to the user.
 */
pub fn start_download() {
    let mut downloaded = 0;
    let total_size = 231231231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}

/**
 * Get IP
 * 
 * Retrieves IP address from a remote (web) data source.
 */
#[tokio::main]
pub async fn get_ip() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}\n", resp);

    Ok(())
}

#[tokio::main]
pub async fn test_api() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://layer1.guru/v1/session";
    // let headers = [("Authorization", "Bearer YOUR_API_KEY"), ("X-Custom-Header", "value")];
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        // .headers(headers.into_iter().collect())
        .body(json_data.to_string())
        .send()
        .await?;

    println!("Status: {}", response.status());
    let response_body = response.text().await?;
    println!("Response body:\n{}", response_body);

    Ok(())
}