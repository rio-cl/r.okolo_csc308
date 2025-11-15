use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("Hello from child process!")
        .output()   
        .expect("Failed to execute child process");

    let result = String::from_utf8_lossy(&output.stdout);
    println!("Child process said: {}", result);
}
