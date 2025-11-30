use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    // Run: echo "Rust Process Management" and capture stdout
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    // Convert stdout to a string
    let text = String::from_utf8_lossy(&output.stdout);

    // Write to output.txt
    let mut file = File::create("output.txt").expect("Failed to create output.txt");
    file.write_all(text.as_bytes()).expect("Failed to write to file");

    println!("Done! Check output.txt");
}
