use std::io;

fn main() {

    println!("Input the sentence you want to slice: ");
    let mut user_string = String::new();
    io::stdin().read_line(&mut user_string).unwrap();

    let slice = user_string.trim();
    let last_word = slice.split_whitespace().last().unwrap();
    println!("The last word is: {}", last_word);
}
