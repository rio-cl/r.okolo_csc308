use std::sync::mpsc;
use std::thread;

fn main() {
  
  let (tx,rx) = mpsc::channel();
  for i in 1..3 {
    thread::spawn(move|| {
        tx.send("Work complete").unwrap();

    });
  }
    println!("Messsage received: {}", rx.recv().unwrap());
}


