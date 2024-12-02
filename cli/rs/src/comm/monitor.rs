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
    sessionid: String,
    method: String,
    resp: String,
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
 * Request JSON
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

/**
 * Respond JSON
 *
 * Make a (remote) API response.
 */
 #[tokio::main]
 async fn response_json(_sessionid: &str, _response: String) -> Result<String, Box<dyn std::error::Error>> {
     /* Set URL (for remote API). */
     let url = format!("{}{}", L1GURU_ENDPOINT, "session");
 
     let exec_response = ExecResponse {
         sessionid: _sessionid.to_string(),
         method: "res".to_string(),
         resp: _response,
     };
 
     let json_string = to_string(&exec_response).unwrap();
 
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

fn _handle_exec(_sessionid: &str, _resp: Vec<Request>) {
// println!("\n***HANDLING (VEC) RESPONSE {:?}", _resp);

    /* Validate response. */
    if (_resp.len() > 0) {
        let exec = &_resp[0].exec;

        // println!("\n***HANDLING (VEC) EXEC {:?}", &exec);

        if (exec == "avax" || exec == "avalanche") {
            let sys_avalanche = cmd::sys::avalanche().expect("Oops! Could NOT execute `avax`.");
            response_json(_sessionid, sys_avalanche);
            return ();
        }

        if (exec == "df") {
            let sys_df = cmd::sys::df().expect("Oops! Could NOT execute `df`.");
            response_json(_sessionid, sys_df);
            return ();
        }

        if (exec == "du") {
            let sys_du = cmd::sys::du().expect("Oops! Could NOT execute `du`.");
            response_json(_sessionid, sys_du);
            return ();
        }

        if (exec == "ls") {
            let sys_ls = cmd::sys::ls().expect("Oops! Could NOT execute `ls`.");
            response_json(_sessionid, sys_ls);
            return ();
        }

        if (exec == "lsblk") {
            let sys_lsblk = cmd::sys::lsblk().expect("Oops! Could NOT execute `lsblk`.");
            response_json(_sessionid, sys_lsblk);
            return ();
        }

        if (exec == "lscpu") {
            let sys_lscpu = cmd::sys::lscpu().expect("Oops! Could NOT execute `lscpu`.");
            response_json(_sessionid, sys_lscpu);
            return ();
        }

        if (exec == "lshw") {
            let sys_lshw = cmd::sys::lshw().expect("Oops! Could NOT execute `lshw`.");
            response_json(_sessionid, sys_lshw);
            return ();
        }

        if (exec == "mem") {
            let sys_mem = cmd::sys::mem().expect("Oops! Could NOT execute `mem`.");
            response_json(_sessionid, sys_mem);
            return ();
        }
    
        if (exec == "ps") {
            let sys_ps = cmd::sys::ps().expect("Oops! Could NOT execute `ps`.");
            response_json(_sessionid, sys_ps);
            return ();
        }
    
        if (exec == "uname") {
            let sys_uname = cmd::sys::get_uname().expect("Oops! Could NOT execute `uname`.");
            response_json(_sessionid, sys_uname);
            return ();
        }

        /*************************************/
        /* HELP */
        /*************************************/

        if (exec == "help") {
            let help = "Oops! Help is temporarily unavailable. Please try again later...".to_string();
            response_json(_sessionid, help);
            return ();
        }

        /*************************************/
        /* UNIMPLEMENTED */
        /*************************************/

        if (exec == "arb" || exec == "arbitrum") {
            let unimplemented = "ERROR! Arbitrum is NOT implemented.".to_string();
            response_json(_sessionid, unimplemented);
            return ();
        }

        if (exec == "base") {
            let unimplemented = "ERROR! Base is NOT implemented.".to_string();
            response_json(_sessionid, unimplemented);
            return ();
        }

        if (exec == "nexa") {
            let unimplemented = "ERROR! Nexa is NOT implemented.".to_string();
            response_json(_sessionid, unimplemented);
            return ();
        }

        if (exec == "op" || exec == "optimism") {
            let unimplemented = "ERROR! Optimism is NOT implemented.".to_string();
            response_json(_sessionid, unimplemented);
            return ();
        }

        if (exec == "sol" || exec == "solana") {
            let unimplemented = "ERROR! Solana is NOT implemented.".to_string();
            response_json(_sessionid, unimplemented);
            return ();
        }

        let unknown = format!("ERROR! [ {} ] is an UNKNOWN command. Try &lt;help&gt; for more options.", exec);
        response_json(_sessionid, unknown);
        return ();
    }

    // let error = "ERROR! A FATAL ERROR OCCURED :(".to_string();
    // response_json(_sessionid, error);
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
        // println!("\n---\n{:?}\n", session_resp); // Output: Person { name: "Jane Doe", age: 25 }

        println!("");
        println!("  SESSION ID -> {}", session_resp.sessionid);
        println!("      ACTION -> {:?}", session_resp.act);
        println!("     REQUEST -> {:?}", session_resp.req);
        println!("     CREATED -> {}", session_resp.created_at);
        println!("  LAST SINCE -> {}", session_resp.last_since);

        unsafe {
            /* Update last since. */
            LAST_SINCE = session_resp.last_since
        }

// println!("\nEXEC-1 -> {:?}", session_resp.req);
        match session_resp.req {
            Some(resp) => _handle_exec(&session_resp.sessionid, resp),
            None => (),
        }
    }
}
