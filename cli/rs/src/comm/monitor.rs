use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

use std::process::exit;
use std::{thread, time};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

/* Initialize constants. */
const L1GURU_ENDPOINT: &str = "https://layer1.guru/v1/";

#[derive(Serialize)]
struct Session {
    sessionid: String,
    since: u32,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct SessionResponse {
    _id: String,
    sessionid: String,
    since: u32,
    created_at: u32,
}


/**
 * Call
 *
 * Make a (remote) API call.
 */
#[tokio::main]
async fn request_json(_sessionid: &str) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}", L1GURU_ENDPOINT, "session");

    let session = Session {
        sessionid: _sessionid.to_string(),
        since: 0,
    };

    let json_string = to_string(&session).unwrap();

    let client = reqwest::Client::new();
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .body(json_string.to_string())
        .send()
        .await?;

    let response_body = response.text().await?;

    /* Return response. */
    Ok(response_body)
}


pub fn cmd(_sessionid: &str) {
    println!("  Waiting for a remote command...\n");

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        println!("  waiting...\n");

        let response = request_json(_sessionid);
        println!("  waiting (JSON)... {}\n", response.as_ref().unwrap());

        // let json_string = r#"{"_id":"some-id","sessionid":"Jane Doe","since":25,"created_at":123}"#;

        let session_resp: SessionResponse = from_str(&response.unwrap()).unwrap();
        println!("{:?}", session_resp); // Output: Person { name: "Jane Doe", age: 25 }
        println!("ID -> {}", session_resp._id);
        println!("NODE ID -> {}", session_resp.sessionid);
        println!("CREATED -> {}", session_resp.created_at);
    }
}
