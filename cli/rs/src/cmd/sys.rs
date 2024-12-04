/* Import modules. */
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use interactive_process::InteractiveProcess;

pub fn df() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("df")
        .arg("-h")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn du() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("du")
        .arg("-hd")
        .arg("2")
        .arg("/home")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
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

pub fn lsblk() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("lsblk")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn lscpu() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("lscpu")
        .arg("-e")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn lshw() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("lshw")
        .output()
        .expect("failed to execute process");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn mem() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("free")
        .arg("-h")
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

pub fn get_uptime() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("uptime")
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
    let output = Command::new("uname")
        .arg("-a")
        .output()
        .expect("uname command failed to complete");

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/**
 * Install Golang
 *
 * Insall the latest stable release of Golang.
 */
pub fn install_golang() -> Result<String, Box<dyn std::error::Error>> {
    // /* Initialize locals. */
    let mut response: String = "".to_string();

    let mut cmd = Command::new("bash");

    let mut proc = InteractiveProcess::new(&mut cmd, |line| {
        println!("    ↳ {}", line.unwrap());
    })
    .unwrap();

    /* Change to (home) directory. */
    proc.send("cd").unwrap();
    sleep(Duration::from_secs(1));

    // proc.send("echo \"export PATH=\$PATH:$HOME/.noderunr/go/bin\" >> .profile").unwrap();
    // sleep(Duration::from_secs(1));

    /* Make (hidden) .noderunr directory (if required). */
    proc.send("mkdir -p .noderunr").unwrap();
    sleep(Duration::from_secs(1));

    /* Change to noderunr directory. */
    proc.send("cd .noderunr").unwrap();
    sleep(Duration::from_secs(1));

    // proc.send("wget https://go.dev/dl/go1.23.3.linux-amd64.tar.gz").unwrap();
    // sleep(Duration::from_millis(1));

    proc.send("export PATH=$PATH:$HOME/.noderunr/go/bin").unwrap();
    sleep(Duration::from_secs(1));

    // proc.send("rm -rf $HOME/.noderunr/go && tar -C $HOME/.noderunr -xzf go1.23.3.linux-amd64.tar.gz").unwrap();
    // sleep(Duration::from_secs(1));

    proc.send("go version").unwrap();
    sleep(Duration::from_secs(1));

    /// We're done with the process, but it is not self-terminating,
    /// so we can't use `proc.wait()`. Instead, we'll take the `Child` from
    /// the `InteractiveProcess` and kill it ourselves.
    proc.close().kill().unwrap();

    Ok(response)
}
