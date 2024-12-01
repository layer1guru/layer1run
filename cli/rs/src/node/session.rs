use serde::{Serialize};
use serde_json::{json, to_string};
use uuid::Uuid;

use crate::api;
use crate::cmd;
use crate::utils;

#[derive(Serialize)]
struct Action {
    action: String,
    pkg: String,
    // age: u32,
}

pub fn new() -> String {
    /* Generate new session id. */
    let sessionid = Uuid::new_v4();

    println!("  NEW session created successfully!\n");
    println!("  [ {} ]\n", sessionid);

    println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
    println!("  https://layer1.run/node/#/{}\n", sessionid);

    // FOR DEVELOPMENT ONLY
    // println!("createdAt {:#}\n", utils::epoch::ms());
    println!("*** IP -> {:?}\n", utils::remote::get_ip());

    // utils::logger::test_log();
    let my_list = cmd::sys::ls().expect("Oops! Could NOT retrieve My List.");
println!("***MY LIST*** {:?}", my_list);

    let myself = cmd::sys::who_am_i();
println!("***MYSELF*** {:?}", myself);
    // let json_data = r#"{"action": "register", "sysinfo": "REDACTED"}"#;
    
    let action = Action {
        action: "register".to_string(),
        pkg: myself.unwrap(),
        // age: 30,
    };

    let json_string = to_string(&action).unwrap();

    // let json_data = format!(r#"{"action": "register", "sysinfo": "REDA"}"#, 
    //     myself.unwrap_or("FAILED! Myself...".to_string()));
println!("***JSON*** {:?}", json_string);
        let response = api::call("session", &json_string);
println!("***RESPONSE*** {:?}", response);

    /* Return session ID. */
    sessionid.to_string()
}
