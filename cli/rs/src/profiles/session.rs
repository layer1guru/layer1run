use serde_json::json;
use uuid::Uuid;

use crate::utils;

pub fn new() -> String {
    /* Generate new session id. */
    let sessionid = Uuid::new_v4();

    println!("  NEW session created successfully!\n");
    println!("  [ {} ]\n", sessionid);

    println!("  Paste the ID 👆 into your Client -OR- click the link below 👇\n");
    println!("  https://layer1.run/{}\n", sessionid);

    // FOR DEVELOPMENT ONLY
    // println!("{:#}\n", json!({
    //     "type": "message",
    //     "content": "Hi there!",
    //     "ip": utils::remote::get_ip(),
    //     "createdAt": utils::epoch::ms(),
    // }));

    /* Return session ID. */
    sessionid.to_string()
}
