use std::collections::HashMap;
mod dictionary;
mod edit_distance;
mod ranker;
mod keyboard_map;
mod edit_cost;


fn main() {
  let user_input: String = String::from("vlue");

  let word_dictionary: HashMap<String, u64> = dictionary::load_dictionary();

  let contains_word: bool = dictionary::lookup_word(user_input.clone(), &word_dictionary);


  if !contains_word {
    let deletion_set: HashMap<String, f64> = edit_distance::deletion(user_input.clone());
    let insertion_set: HashMap<String, f64> = edit_distance::insertion(user_input.clone());
    let substitution_set: HashMap<String, f64> = edit_distance::substitution(user_input.clone());
    let transposition_set: HashMap<String, f64> = edit_distance::transposition(user_input.clone());

    let in_dictionary: HashMap<String, f64> = edit_distance::search_dictionary(&word_dictionary, deletion_set, insertion_set, substitution_set, transposition_set);

    let in_dictionary_frequency_score: Vec<(String, u64, f64)> = ranker::combine_frequency_score(in_dictionary, &word_dictionary);

    let scored_candidates: Vec<(String, u64, f64)> = edit_cost::edit_score(in_dictionary_frequency_score);

    let freq_ranking: Vec<(String, u64, f64)> = ranker::score_ranking(scored_candidates);
    

    println!("{:?}", freq_ranking);
  }
  else {
    println!("'{}' is in the dictionary", user_input);
  }

  
}