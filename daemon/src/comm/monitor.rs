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

#[derive(Debug, Serialize)]
struct ExecResponse {
    // sessionid: String,
    method: String,
    requestid: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct Log {
    body: String,
    created_at: u64, // milliseconds
}

#[derive(Debug, Deserialize)]
struct Request {
    cmd: String,
    created_at: u64, // milliseconds
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize)]
struct ApiRequestResponse {
    success: bool,
    totalCount: u32,
    data: Option<Vec<RequestResponse>>,
    cursor: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize)]
struct RequestResponse {
    requestid: String,
    sessionid: String,
    cmd: String,
    payload: String,
    status: u8,
    createdAt: u64, // milliseconds
    // updatedAt: u64, // milliseconds
    // last_since: u64 // milliseconds
}

#[derive(Serialize)]
struct Session {
    sessionid: String,
    since: u64, // milliseconds
}

#[derive(Debug, Default, Deserialize)]
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
 * Request JSON
 *
 * Make a (remote) API call.
 */
#[tokio::main]
async fn request_json(sessionid: &str, _since: u64) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}{}", L1GURU_ENDPOINT, "command/request?since=", _since);

    /* Initialize client. */
    let client = reqwest::Client::new();

    /* Set (authorization) token. */
    let token = format!("Bearer {}", sessionid);

    /* Send API request. */
    let response = client.get(url)
        .header("Authorization", token)
        .header("content-type", "application/json")
        .send()
        .await?;

    /* Handle (API request) response */
    let response_body = response.text().await?;

    /* Return response. */
    Ok(response_body)
}

/**
 * Respond JSON
 *
 * Make a (remote) API response.
 */
#[tokio::main]
async fn response_json(sessionid: &str, requestid: &str, _response: String) -> Result<String, Box<dyn std::error::Error>> {
    /* Set URL (for remote API). */
    let url = format!("{}{}", L1GURU_ENDPOINT, "command");
 
    /* Build response. */
    let exec_response = ExecResponse {
        method: "res".to_string(),
        requestid: requestid.to_string(),
        body: _response,
    };
 
    /* Initialize JSON string. */
    let json_string = to_string(&exec_response).unwrap();
 
    /* Initialize client. */
    let client = reqwest::Client::new();

    /* Make (remote) data request. */
    let response = client.post(url)
        .header("Authorization", format!("Bearer {}", sessionid.to_string()))
        .header("content-type", "application/json")
        .body(json_string.to_string())
        .send()
        .await?;
 
    /* Parse data result. */
    // let response_body = response.json().await?;
    let response_body: String = response.text().await?;
// println!("RESPONSE BODY {:?}", response_body);
    /* Return response. */
    Ok(response_body)
}

/**
 * Handle Execution
 * 
 * Perform execution of the user's specified command against the local system.
 */
fn _handle_exec(_resp: RequestResponse) {
// println!("\n***HANDLING (VEC) RESPONSE {:?}", _resp);

    /* Set command. */
    let cmd = &_resp.cmd;
// println!("\n***HANDLING (VEC) CMD {:?}", cmd);

    /* Set session ID. */
    let sessionid = &_resp.sessionid;

    /* Set session ID. */
    let requestid = &_resp.requestid;

    if (cmd == "avax" || cmd == "avalanche") {
        let response = cmd::network::avax().expect("Oops! Could NOT execute `avax`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "install avax" || cmd == "install avalanche") {
        let response = cmd::network::avax_install().expect("Oops! Could NOT execute `avax_install`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "start avax" || cmd == "start avalanche") {
        let response = cmd::network::avax_start().expect("Oops! Could NOT execute `avax_start`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "avax status" || cmd == "avalanche status") {
        let response = cmd::network::avax_status().expect("Oops! Could NOT execute `avax_status`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "build avax" || cmd == "build avalanche") {
        let response = cmd::network::build_avalanche().expect("Oops! Could NOT execute `install avax`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "df") {
        let response = cmd::sys::df().expect("Oops! Could NOT execute `df`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "du") {
        let response = cmd::sys::du().expect("Oops! Could NOT execute `du`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "install go" || cmd == "install golang") {
        let response = cmd::sys::install_golang().expect("Oops! Could NOT execute `install go`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "ls") {
        let response = cmd::sys::ls().expect("Oops! Could NOT execute `ls`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "lsblk") {
        let response = cmd::sys::lsblk().expect("Oops! Could NOT execute `lsblk`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "lscpu") {
        let response = cmd::sys::lscpu().expect("Oops! Could NOT execute `lscpu`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "lshw") {
        let response = cmd::sys::lshw().expect("Oops! Could NOT execute `lshw`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "mem") {
        let response = cmd::sys::mem().expect("Oops! Could NOT execute `mem`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "ps") {
        let response = cmd::sys::ps().expect("Oops! Could NOT execute `ps`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "profiler") {
        let response = cmd::sys::system_profiler().expect("Oops! Could NOT execute `system_profiler`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "uname") {
        let response = cmd::sys::get_uname().expect("Oops! Could NOT execute `uname`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "uptime") {
        let response = cmd::sys::get_uptime().expect("Oops! Could NOT execute `uptime`.");
        response_json(sessionid, requestid, response);
        return ();
    }

    /*************************************/
    /* HELP */
    /*************************************/

    if (cmd == "help") {
        let response = "Oops! Help is temporarily unavailable. Please try again later...".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    /*************************************/
    /* UNIMPLEMENTED */
    /*************************************/

    if (cmd == "arb" || cmd == "arbitrum") {
        let response = "ERROR! Arbitrum is NOT implemented.".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "base") {
        let response = "ERROR! Base is NOT implemented.".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "nexa") {
        let response = "ERROR! Nexa is NOT implemented.".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "op" || cmd == "optimism") {
        let response = "ERROR! Optimism is NOT implemented.".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    if (cmd == "sol" || cmd == "solana") {
        let response = "ERROR! Solana is NOT implemented.".to_string();
        response_json(sessionid, requestid, response);
        return ();
    }

    let response = format!("ERROR! [ {} ] is an UNKNOWN command. Try &lt;help&gt; for more options.", cmd);
    response_json(sessionid, requestid, response);
    ()
    // return ();
}

pub fn by_session(sessionid: &str) {
    println!("\n  Waiting for Client command...\n");

    let mut response: Result<String, Box<dyn std::error::Error>>;

    /* Start inifinite loop. */
    loop {
        let ten_seconds = time::Duration::from_millis(10000);
        let now = time::Instant::now();
        
        thread::sleep(ten_seconds);
        
        assert!(now.elapsed() >= ten_seconds);

        unsafe {
            /* Make (remote) JSON (data) request. */
            response = request_json(sessionid, LAST_SINCE);
// println!("\nRAW---\n{:?}\n", response);
        }

        // let session_resp: Result<_, Box<dyn std::error::Error>>;
        let mut session_resp: Result<ApiRequestResponse, serde_json::Error> = Ok(ApiRequestResponse::default());
        // let session_resp = SessionResponse::default();

        match(&response) {
            Ok(_data) => {
                session_resp = from_str(_data);
            },
            Err(_) => println!("\n  ERROR: Failed to receive a response from API server."),
        }
// println!("\nSR---\n{:?}\n", session_resp);

        let mut remote_data: ApiRequestResponse = ApiRequestResponse::default();
        // let mut remote_data: Option<SessionResponse> = None;
        // let mut remote_data: SessionResponse;

        match(session_resp) {
            Ok(_data) => {
                // remote_data = session_resp.unwrap();
                remote_data = _data;

                unsafe {
                    /* Update last since. */
                    // LAST_SINCE = remote_data.last_since
                    // LAST_SINCE = remote_data.data[0].updatedAt
                }
            },
            // Err(_) => println!("ERROR: Failed to receive any remote data."),
            Err(_) => (),
        }
// println!("\nRD---\n{:?}\n", remote_data); // Output: Person { name: "Jane Doe", age: 25 }

// println!("");
// println!("  SESSION ID -> {}", remote_data.data[0].sessionid);
// println!("      ACTION -> {:?}", remote_data.data[0].act);
// println!("     REQUEST -> {:?}", remote_data.data[0].req);
// println!("     CREATED -> {}", remote_data.data[0].created_at);
// println!("  LAST SINCE -> {}", remote_data.data[0].updatedAt);

        let mut request_data = RequestResponse::default();
        // let mut request_data_alt = Some(RequestResponse::default());

        match(remote_data.data) {
            Some(_data) => {
                // remote_data = session_resp.unwrap();
                // request_data = _data.get(0);

                /* Validate data. */
                if (_data.len() > 0) {
                    // _handle_exec(_data)
                    
                    /* Clone data. */
                    let mut cloned = _data;

                    /* Set (cloned) data. */
                    request_data = cloned.remove(0);
                }
            },
            // Err(_) => println!("ERROR: Failed to receive any remote data."),
            None => (),
        }

        /* Validate request ID. */
        if (!request_data.requestid.is_empty()) {
// println!("\nRQD---\n{:?}\n", request_data);
// println!("\nSID---\n{:?}\n", request_data.sessionid);
// println!("\nCMD---\n{:?}\n", request_data.cmd);
// println!("\n@TS---\n{:?}\n", request_data.createdAt);

            unsafe {
                /* Update last since. */
                LAST_SINCE = request_data.createdAt
            }

            /* Handle execution. */
            _handle_exec(request_data)
        }
    }
}
