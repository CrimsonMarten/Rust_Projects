use std::io;

fn main() {    
    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Invalid string!");

    let mut translation = String::new();

    for word in sentence.split_whitespace() {
        translation.push_str(&convert_word(word)[..]);
    }

    println!("Translation: {}", translation.trim_end());
    
}

fn convert_word(word: &str) -> String {
    let vowels = vec!["a", "e", "i", "o", "u"];
    let mut pig_word;
    if vowels.contains(&&word[0..1]) {
        pig_word = String::from(word);
        pig_word.push_str("-hay ");
    }
    else {
        pig_word = String::from(&word[1..]);
        let suffix = format!("-{}{} ", &word[0..1], "ay");
        pig_word.push_str(&suffix);
    }

    pig_word
}