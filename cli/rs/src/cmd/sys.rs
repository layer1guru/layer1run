/* Import modules. */
use std::io::{BufReader, BufRead};
use std::io::{self, Write};
use std::process::{Command, Stdio};

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