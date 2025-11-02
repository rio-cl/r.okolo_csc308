use std::io;

struct Sentence {
    text: String,
}

impl Sentence {
    fn new(text: String) -> Self {
        Sentence { text }
    }
s
    fn get_words(&self) -> Vec<&str> {
        self.text
            .split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .filter(|word| !word.is_empty())
            .collect()
    }

    fn longest_word(&self) -> Option<&str> {
        let words = self.get_words();
        words.iter().max_by_key(|w| w.len()).copied()
    }

    fn shortest_word(&self) -> Option<&str> {
        let words = self.get_words();
        words.iter().min_by_key(|w| w.len()).copied()
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a sentence: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let sentence = Sentence::new(input.trim().to_string());

    println!("\n--- Sentence Analysis ---");
    println!("Sentence: {}", sentence.text);

    match sentence.longest_word() {
        Some(word) => println!("Longest word: {}", word),
        None => println!("No words found."),
    }

    match sentence.shortest_word() {
        Some(word) => println!("Shortest word: {}", word),
        None => println!("No words found."),
    }
}
