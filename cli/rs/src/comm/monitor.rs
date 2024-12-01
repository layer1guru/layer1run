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
    since: u32, // seconds
}

#[derive(Debug, Deserialize)]
struct Action {
    actionid: Option<String>,
    body: Option<String>,
    target: Option<String>,
    created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize)]
struct Log {
    body: String,
    created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize)]
struct Request {
    body: String,
    created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize)]
struct SessionResponse {
    sessionid: String,
    act: Option<Vec<Action>>,
    log: Option<Vec<Action>>,
    req: Option<Vec<Request>>,
    res: Option<Vec<Action>>,
    rpt: Option<Vec<Action>>,
    created_at: u32, // seconds
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


pub fn by_session(_sessionid: &str) {
    println!("\n  Waiting for Client command...");

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        /* Make (remote) JSON (data) request. */
        let response = request_json(_sessionid);
        // println!("\n  ...handler -> {}", response.as_ref().unwrap());

        // let json_string = r#"{"_id":"some-id","sessionid":"Jane Doe","since":25,"created_at":123}"#;

        let session_resp: SessionResponse = from_str(&response.unwrap()).unwrap();
        println!("\n---\n{:?}\n", session_resp); // Output: Person { name: "Jane Doe", age: 25 }

        println!("  SESSION ID -> {}", session_resp.sessionid);
        println!("  ACTION -> {:?}", session_resp.act);
        println!("  REQUEST -> {:?}", session_resp.req);
        println!("  CREATED -> {}", session_resp.created_at);
    }
}
