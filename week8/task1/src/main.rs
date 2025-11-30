use std::process::{Command, Stdio};
use std::io::Read;

fn main() {
    let mut child1 = Command::new("sleep").arg("5").spawn().expect("Failed to spawn");
    let mut child2 = Command::new("ls").arg("-la").spawn().expect("Failed to spawn");
    let mut child3 = Command::new("echo").arg("Hello from child").stdout(Stdio::piped()).spawn().expect("Failed to spawn echo");

    let mut output = String::new();
    child3.stdout.as_mut().unwrap().read_to_string(&mut output).unwrap();

    println!("Child3 Output: {}", output);
}
