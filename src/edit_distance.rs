use std::{collections::HashMap, hash::Hash, usize};
use crate::{edit_distance, keyboard_map};


pub fn deletion(user_input: String, mut edit_cost: f64) -> HashMap<String, f64> {
  let mut deletion_set: HashMap<String, f64> = HashMap::new();
  edit_cost += 2.0;

  let chars: Vec<char> = user_input.chars().collect();
  let chars_length = chars.len();

  for c in 0..chars_length {
    let mut temp_chars: Vec<char> = chars.clone();

    temp_chars.remove(c);
    let modified_string: String = temp_chars.iter().collect();
    deletion_set.insert(modified_string, edit_cost);
  }

  deletion_set
}

pub fn insertion(mut user_input: String, mut edit_cost: f64) -> HashMap<String, f64> {
  let mut insertion_set: HashMap<String, f64> = HashMap::new();
  let string_length: usize = user_input.len();
  edit_cost += 2.0;

  for c in 0..string_length+1 {
    for l in 'a'..='z' {
      user_input.insert(c, l);
      insertion_set.insert(user_input.clone(), edit_cost);
      user_input.remove(c);
    }
  }

  insertion_set
}

pub fn substitution(mut user_input: String, mut edit_cost: f64) -> HashMap<String, f64> {
  let mut substitution_set: HashMap<String, f64> = HashMap::new();
  let string_length: usize = user_input.len();
  let mut char_adjacent: bool;

  for c in 0..string_length {
    let original_input: String = user_input[c..c+1].to_string();

    for l in 'a'..='z' {
      let mut buf: [u8; 4] = [0; 4];
      user_input.replace_range(c..c+1, l.encode_utf8(&mut buf));

      char_adjacent = keyboard_map::is_adjacent(original_input.chars().next().unwrap(), l);

      if char_adjacent {
        edit_cost += 1.0;
      }
      else {
        edit_cost += 2.0;
      }

      substitution_set.insert(user_input.clone(), edit_cost);


      user_input.replace_range(c..c+1, &original_input);

    }
  }

  substitution_set
}

pub fn transposition(user_input: String, mut edit_cost: f64) -> HashMap<String, f64> {
  let mut transposition_set: HashMap<String, f64> = HashMap::new();
  let string_length: usize = user_input.len();
  let mut string_vector: Vec<char> = user_input.chars().collect();
  edit_cost += 0.8; // most common typing mistake

  for c in 0..string_length-1 {
    string_vector.swap(c, c+1);
    transposition_set.insert(string_vector.iter().collect(), edit_cost);
    string_vector.swap(c, c+1);
  }

  transposition_set
}

pub fn generate_all_edits(user_input: String, edit_cost: f64) -> HashMap<String, f64> {
  let mut all_candidates: HashMap<String, f64> = HashMap::new();

  all_candidates.extend(deletion(user_input.clone(), edit_cost));
  all_candidates.extend(insertion(user_input.clone(), edit_cost));
  all_candidates.extend(substitution(user_input.clone(), edit_cost));
  all_candidates.extend(transposition(user_input.clone(), edit_cost));

  all_candidates
}

pub fn search_dictionary(
  word_dictionary: &HashMap<String, u64>, 
  all_candidates: HashMap<String, f64>) 
  -> HashMap<String, f64> {
  let all_vector: Vec<HashMap<String, f64>> = vec![all_candidates];

  let all_norvig_sets: HashMap<String, f64> = all_vector
    .into_iter()
    .flatten()
    .collect();

  let in_dictionary: HashMap<String, f64> = all_norvig_sets
    .iter()
    .filter(|(key, _value)| word_dictionary.contains_key(*key))
    .map(|(key, value)| (key.clone(), *value))
    .collect();


  in_dictionary
}