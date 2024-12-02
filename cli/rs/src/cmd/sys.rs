/* Import modules. */
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
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

pub fn ping2() {
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn process");

    let mut stdout = child.stdout.as_mut().unwrap();
    let mut stderr = child.stderr.as_mut().unwrap();

    // let stdout = String::from_utf8(stdout).unwrap();
    println!("{:?}", stdout);

    // io::stdout().write_all(&mut stdout).unwrap();
    // io::stderr().write_all(&mut stderr).unwrap();
}

pub fn ls() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ls")
        // .arg("~")
        .arg("/home")
        .arg("-l")
        .arg("-a")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn ps() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("ps")
        .arg("aux")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn get_uname() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/**
 * Get Release
 *
 * Request the system release details.
 */
pub fn get_release() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("lsb_release")
        .arg("-a")
        .output()
        .expect("lsb_release command failed to complete");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}