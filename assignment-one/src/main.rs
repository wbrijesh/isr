use std::fs::File;
use std::io::{self, Read};

fn remove_word_from_sentence(sentence: &str, word_to_remove: &str) -> String {
    let word_to_remove_lower = word_to_remove.to_lowercase();

    sentence
        .split_whitespace() 
        .filter(|&word| word.to_lowercase() != word_to_remove_lower) 
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    let words_to_remove = ["sometimes", "on", "is", "that", "to"];

    let file_path = "../input.txt";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);


    for i in 0..words_to_remove.len() {
        contents = remove_word_from_sentence(&contents, &words_to_remove[i]);
    }

    println!("{}", contents);

    Ok(())    
}