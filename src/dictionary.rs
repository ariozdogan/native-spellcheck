use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub fn load_dictionary() -> HashMap<String, u64> {
  let mut word_dictionary: HashMap<String, u64> = HashMap::new();

  let file: File = File::open("data/count_1w.txt").expect("Could not open file");
  let reader: BufReader<File> = BufReader::new(file);

  for line in reader.lines() {
    let line: String = line.expect("Could not read line");

    if let Some((word, frequency)) = line.split_once('\t') { // split each line into two strings
      let frequency: u64 = frequency.parse::<u64>().expect("Not a valid number"); // frequency: String -> u64

      word_dictionary.insert(String::from(word),frequency);
    }
  }

  word_dictionary
}

pub fn lookup_word(user_input: String, word_dictionary: HashMap<String, u64>) -> bool {
  let contains_word: bool;

  let cleaned_input: String = user_input // strip spaces and punctuation, and turn to lowercase
    .chars()
    .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation())
    .map(|c| c.to_ascii_lowercase())
    .collect::<String>(); 

  if word_dictionary.contains_key(&cleaned_input) {
    contains_word = true;
  }
  else {
    contains_word = false;
  }

  contains_word 

}