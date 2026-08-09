use std::collections::{HashMap, HashSet};
mod dictionary;
mod edit_distance;
mod ranker;
mod keyboard_map;
mod edit_cost;

fn main() {
  let user_input: String = String::from("jig");

  let word_dictionary: HashMap<String, u64> = dictionary::load_dictionary();

  let contains_word: bool = dictionary::lookup_word(user_input.clone(), &word_dictionary);

  let mut in_dictionary: HashSet<String> = HashSet::new();

  if !contains_word {
    let deletion_set: HashSet<String> = edit_distance::deletion(user_input.clone());
    let insertion_set: HashSet<String> = edit_distance::insertion(user_input.clone());
    let substitution_set: HashSet<String> = edit_distance::substitution(user_input.clone());
    let transposition_set: HashSet<String> = edit_distance::transposition(user_input.clone());

    in_dictionary = edit_distance::search_dictionary(&word_dictionary, deletion_set, insertion_set, substitution_set, transposition_set);

    let word_frequency: HashMap<String, u64> = ranker::in_dictionary_frequency(in_dictionary, &word_dictionary);

    let freq_ranking: Vec<(String, u64)> = ranker::frequency_ranking(word_frequency);
    

    println!("{:?}", freq_ranking);
  }
  else {
    println!("'{}' is in the dictionary", user_input);
  }

  
}