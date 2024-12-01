use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};
// use uuid::Uuid;

use crate::api;
use crate::comm;
use crate::cmd;
use crate::utils;

#[derive(Serialize)]
struct Registration {
    method: String,
    ip: String,
    release: String,
}

#[derive(Debug, Deserialize)]
struct RegistrationResponse {
    sessionid: String,
    success: bool,
    created_at: u32, // seconds
}


/**
 * New Session
 *
 * Request a new session from the API server.
 */
pub fn new() -> String {
    /* Request release. */
    let release = cmd::sys::get_release().unwrap();
// println!("RELEASE {:?}", release);
    
    /* Request IP address. */
    let response = utils::ip::get().unwrap();

    /* Set IP address. */
    let ip = &response["origin"];
// println!("IP -> {}", ip);

    /* Build (registration) package. */
    let pkg = Registration {
        method: "reg".to_string(),
        ip: ip.to_string(),
        release: release.to_string(),
    };

    /* Encode to JSON. */
    let json_string = to_string(&pkg).unwrap();

    /* Make (remote) request. */
    let response = api::call("session", &json_string);
// println!("\nRESPONSE (json) {:?}", response);

    /* Parse (registration) response. */
    let response: RegistrationResponse = from_str(&response.unwrap()).unwrap();
// println!("\nRESPONSE (session) {:?}", response);

    /* Set session id. */
    let sessionid = response.sessionid;

    println!("  NEW session created successfully!\n");
    println!("  [ {} ]\n", sessionid);

    println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
    println!("  https://layer1.run/sid/#/{}", sessionid);

    /* Start monitoring session. */
    comm::monitor::by_session(&sessionid);

    /* Return session ID. */
    sessionid.to_string()
}
