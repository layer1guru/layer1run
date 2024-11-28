use std::io::{BufReader, BufRead};
use std::process::{Command, Stdio};


/**
 * Ping
 * 
 * Starts a long-lived ping process on the provided destination.
 */
pub fn ping() {
    let mut child = Command::new("ping")
        .arg("www.yahoo.com")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Oops! Failed to execute child.");

    /* Initialize output for child. */ 
    let stdout = child.stdout
        .as_mut()
        .expect("Oops! Failed to initialize output for child.");
    
    /* Initialize intput buffer. */
    let stdout_reader = BufReader::new(stdout);
    
    /* Handle line inputs. */
    let stdout_lines = stdout_reader
        .lines();

    /* Handle output reader buffer. */
    for line in stdout_lines {
        println!("Read -> {:?}", line);
    }

    /* Wait for child. */
    // TODO: How can we retrieve the final output?
    let output = child
        .wait_with_output()
        .expect("Oops! Failed to wait for child.");
    assert!(output.status.success());
    println!("Final output -> {:?}\n", output);
}

pub fn ls() {
    let output = Command::new("ls")
        .arg("-l")
        .spawn()
        .expect("ls command failed to start")
        .wait_with_output()
        .expect("ls command failed to complete");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn who_am_i() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("lsb_release")
        .arg("-a")
        .spawn()
        .expect("lsb_release command failed to start")
        .wait_with_output()
        .expect("lsb_release command failed to complete");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}