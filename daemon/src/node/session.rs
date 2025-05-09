use serde::{Deserialize, Serialize};
use serde_json::{from_str, json, to_string};

use crate::api;
use crate::comm;
use crate::cmd;
use crate::utils;

#[derive(Serialize)]
struct Registration {
    // method: String,
    ip: String,
    release: String,
    uptime: String,
    cpu: String,
    mem: String,
    profile: String,
}

#[derive(Debug, Default, Deserialize)]
struct RegistrationResponse {
    success: bool,
    result: Session,
    // created_at: u32, // seconds
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Deserialize)]
struct Session {
    sessionid: String,
    hasAuth: bool,
    createdAt: u32,
}

/**
 * New Session
 *
 * Request a new session from the API server.
 */
pub fn new() -> String {
    /* Initialize IP address handler. */
    let mut ip: String;

    /* Request IP address. */
    let response = utils::ip::get();

    /* Set IP address. */
    match response {
        Ok(_resp) => {
            ip = _resp["origin"].clone();
        },
        Err(err) => {
            ip = err.to_string();
        }
    }    

    /* Request release. */
    let release: String = cmd::sys::get_release().unwrap();
    
    /* Request uptime. */
    let uptime: String = cmd::sys::get_uptime().unwrap();
    
    /* Request cpu. */
    let cpu: String = cmd::sys::lscpu().unwrap();
    
    /* Request mem. */
    let mem: String = cmd::sys::mem().unwrap();
    
    /* Request system profile. */
    let profile: String = cmd::sys::system_profiler().unwrap();
    
    /* Build (registration) package. */
    let pkg: Registration = Registration {
        ip: ip.to_string(),
        release: release.to_string(),
        uptime: uptime.to_string(),
        cpu: cpu.to_string(),
        mem: mem.to_string(),
        profile: profile.to_string(),
    };

    /* Encode to JSON. */
    let json_string: String = to_string(&pkg).unwrap();

    /* Make (remote) request. */
    let response = api::call("session", &json_string);

    /* Initialize (registration) response handler. */
    let mut reg_response: Result<RegistrationResponse, serde_json::Error> = Ok(RegistrationResponse::default());

    /* Parse (registration) response. */
    match(&response) {
        Ok(_data) => {
            reg_response = from_str(_data);
        },
        Err(_) => println!("  Ugh! Your node registration failed!\n  Sorry about that, please try again...\n\n")
    }

    /* Initialize registration handler. */
    let mut registration: RegistrationResponse = RegistrationResponse::default();

    /* Parse (registration) response. */
    match(reg_response) {
        Ok(_data) => {
            /* Handle (session) data result. */
            let session: Session = _data.result;

            /* Set session ID. */
            let sessionid: String = session.sessionid;

            println!("    ✨ NEW session created successfully ✨");
            println!("  .-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-.");
            println!("  |                                                   |");
            println!("  :        {}       :", sessionid);
            println!("  |                                                   |");
            println!("  `-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-Current SessionID-=-'\n");
            
            println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
            println!("  https://layer1.run/id/#/{} ↗️", sessionid);

            /* Start monitoring session. */
            comm::monitor::by_session(&sessionid);

            /* Return session ID. */
            sessionid.to_string()
        },
        Err(_) => ("".to_string())
    }
}
