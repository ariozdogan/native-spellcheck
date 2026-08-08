use std::collections::HashMap;
use spellcheck::dictionary::load_dictionary;
use spellcheck::dictionary::lookup_word;

fn main() {
  let words: HashMap<String, u64> = load_dictionary();

  println!("{:?}", words);

  let user_input: String = "".to_string();

  let contains_word: bool = lookup_word(user_input, words);

  println!("{}", contains_word);

}