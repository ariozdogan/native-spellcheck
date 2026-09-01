pub mod dictionary;
pub mod edit_distance;
pub mod ranker;
pub mod keyboard_map;
pub mod edit_cost;

use std::collections::HashMap;

pub fn placeholder() -> String {
  "lib is wired up".to_string()
}

pub fn handle_completed_word(user_word: &String) -> String {
  if user_word.is_empty() {
    return user_word.clone();
  }

  let word_dictionary: HashMap<String, u64> = dictionary::load_dictionary();
  let contains_word: bool = dictionary::lookup_word(user_word.clone(), &word_dictionary);
  let edit_cost: f64 = 0.0;

  if contains_word {
    return user_word.clone();
  }

  let all_candidates: HashMap<String, f64> = edit_distance::generate_all_edits(user_word, edit_cost);
  let mut all_candidates_2: HashMap<String, f64> = HashMap::new();

  let mut in_dictionary: HashMap<String, f64> = edit_distance::search_dictionary(&word_dictionary, all_candidates.clone());

  if in_dictionary.len() == 0 {
    for key in all_candidates.keys() {
      let all_candidates_iteration: &str = &key;
      let generate_second_edit: HashMap<String, f64> = edit_distance::generate_all_edits
      (&all_candidates_iteration.to_string(), edit_cost);
      all_candidates_2.extend(generate_second_edit);
    }
    in_dictionary = edit_distance::search_dictionary(&word_dictionary, all_candidates_2)
  }

  let in_dictionary_frequency_score: Vec<(String, u64, f64)> = ranker::combine_frequency_score(in_dictionary, &word_dictionary);

  let scored_candidates: Vec<(String, u64, f64)> = edit_cost::edit_score(in_dictionary_frequency_score);

  let freq_ranking = ranker::score_ranking(scored_candidates);

  if freq_ranking.is_empty() {
    return user_word.clone();
  }

  return freq_ranking[0].0.clone()
}