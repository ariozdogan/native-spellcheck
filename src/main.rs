use std::{collections::HashMap, hash::Hash};
use spellcheck::edit_distance::generate_all_edits;
mod dictionary;
mod edit_distance;
mod ranker;
mod keyboard_map;
mod edit_cost;


fn main() {
  let user_input: String = String::from("sdofi");
  let word_dictionary: HashMap<String, u64> = dictionary::load_dictionary();
  let contains_word: bool = dictionary::lookup_word(user_input.clone(), &word_dictionary);
  let edit_cost: f64 = 0.0;


  if !contains_word {
    let all_candidates: HashMap<String, f64> = generate_all_edits(user_input, edit_cost);
    let mut all_candidates_2: HashMap<String, f64> = HashMap::new();

    let mut in_dictionary: HashMap<String, f64> = edit_distance::search_dictionary(&word_dictionary, all_candidates.clone());

    if in_dictionary.len() == 0 {
      for key in all_candidates.keys() {
        let all_candidates_iteration: &str = &key;
        let generate_second_edit: HashMap<String, f64> = generate_all_edits(all_candidates_iteration.to_string(), edit_cost);
        all_candidates_2.extend(generate_second_edit);
      }
      in_dictionary = edit_distance::search_dictionary(&word_dictionary, all_candidates_2)
    }

    let in_dictionary_frequency_score: Vec<(String, u64, f64)> = ranker::combine_frequency_score(in_dictionary, &word_dictionary);

    let scored_candidates: Vec<(String, u64, f64)> = edit_cost::edit_score(in_dictionary_frequency_score);

    let freq_ranking: Vec<(String, u64, f64)> = ranker::score_ranking(scored_candidates);
    

    println!("{:?}", freq_ranking);
  }
  else {
    println!("'{}' is in the dictionary", user_input);
  }

  
}