fn main() {
    let mut message = String::from("Hello");
    
    one_more(&message);
    show_message(&mut message);
    add_note(&mut message);
    
    println!("Final message: {}", message);
}

fn show_message(msg: &mut String) {
    msg.push_str(", my guy");
    println!("Current message: {}", msg);
}

fn add_note(msg: &mut String) {
    msg.push_str(", world!");
}

fn one_more(msg: &String) {
    println!("{}", msg)
}