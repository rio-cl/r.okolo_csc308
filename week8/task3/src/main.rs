use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;

fn main() {
    // Spawn long-running child process
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::null()) // We don’t need the output
        .spawn()
        .expect("Failed to spawn ping");

    println!("Spawned ping with PID: {}", child.id());
    println!("Use 'top' or 'ps' to find the process now...");

    // Wait for 5 seconds while ping is running
    thread::sleep(Duration::from_secs(5));

    // Kill the child process after 5 seconds
    println!("Killing ping process...");
    let _ = child.kill();

    // Wait for the killed process to exit
    let status = child.wait().expect("Failed to wait for child");

    println!("Child exited with status: {}", status);
}
