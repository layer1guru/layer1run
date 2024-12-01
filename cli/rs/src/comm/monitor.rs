use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

use std::process::exit;
use std::{thread, time};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

use crate::cmd;

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
    exec: String,
    created_at: u64, // milliseconds
}

#[derive(Serialize)]
struct Session {
    sessionid: String,
    since: u64, // milliseconds
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
    last_since: u64 // milliseconds
}

/* Initialize constants. */
const L1GURU_ENDPOINT: &str = "https://layer1.guru/v1/";

/* Initialize globals. */
static mut LAST_SINCE: u64 = 1;


/**
 * Call
 *
 * Make a (remote) API call.
 */
#[tokio::main]
async fn request_json(_sessionid: &str, _since: u64) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}", L1GURU_ENDPOINT, "session");

    let session = Session {
        sessionid: _sessionid.to_string(),
        since: _since,
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

fn _handle_exec(_resp: Vec<Request>) {
println!("\n***HANDLING (VEC) RESPONSE {:?}", _resp);

    /* Validate response. */
    if (_resp.len() > 0) {
        println!("\n***HANDLING (VEC) EXEC {:?}", _resp[0].exec);

        // if (session_resp.req) {
    //     println!("\nEXEC-2 -> {:?}", session_resp.req.unwrap());
    // }
            // if (session_resp.req.unwrap().exec) {
    
                // utils::logger::test_log();
                let sys_ls = cmd::sys::ls().expect("Oops! Could NOT execute `ls`.");
    println!("\nLS -> {:?}", sys_ls);
            // }
    
    }
}

pub fn by_session(_sessionid: &str) {
    println!("\n  Waiting for Client command...");

    let mut response: Result<String, Box<dyn std::error::Error>>;

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        unsafe {
            /* Make (remote) JSON (data) request. */
            response = request_json(_sessionid, LAST_SINCE);
            // println!("\n  ...handler -> {}", response.as_ref().unwrap());
        }

        // let json_string = r#"{"_id":"some-id","sessionid":"Jane Doe","since":25,"created_at":123}"#;

        let session_resp: SessionResponse = from_str(&response.unwrap()).unwrap();
        println!("\n---\n{:?}\n", session_resp); // Output: Person { name: "Jane Doe", age: 25 }

        println!("  SESSION ID -> {}", session_resp.sessionid);
        println!("      ACTION -> {:?}", session_resp.act);
        println!("     REQUEST -> {:?}", session_resp.req);
        println!("     CREATED -> {}", session_resp.created_at);
        println!("  LAST SINCE -> {}", session_resp.last_since);

        unsafe {
            /* Update last since. */
            LAST_SINCE = session_resp.last_since
        }

println!("\nEXEC-1 -> {:?}", session_resp.req);
match session_resp.req {
    Some(resp) => _handle_exec(resp),
    None => println!("\nFOUND NOTHING..."),
}
    }
}
