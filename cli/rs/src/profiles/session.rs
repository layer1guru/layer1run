use serde_json::json;
use uuid::Uuid;

use crate::api;
use crate::commander;
use crate::utils;

pub fn new() -> String {
    /* Generate new session id. */
    let sessionid = Uuid::new_v4();

    println!("  NEW session created successfully!\n");
    println!("  [ {} ]\n", sessionid);

    println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
    println!("  https://layer1.run/{}\n", sessionid);

    // FOR DEVELOPMENT ONLY
    // println!("createdAt {:#}\n", utils::epoch::ms());
    println!("*** IP -> {:?}\n", utils::remote::get_ip());
    println!("*** API -> {:?}\n", utils::remote::test_api());

    // utils::logger::test_log();
    // commander::sys::who_am_i();

    let json_data = r#"{"action": "register", "sysinfo": "REDACTED"}"#;
    let response = api::call("session", json_data);
println!("***RESPONSE*** {:?}", response);

    /* Return session ID. */
    sessionid.to_string()
}
