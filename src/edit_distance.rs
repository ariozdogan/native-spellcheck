use std::{collections::{HashMap, HashSet}, usize};


pub fn deletion(user_input: String) -> HashSet<String> {
  let mut deletion_set: HashSet<String> = HashSet::new();

  let chars: Vec<char> = user_input.chars().collect();
  let chars_length = chars.len();

  for c in 0..chars_length {
    let mut temp_chars: Vec<char> = chars.clone();

    temp_chars.remove(c);
    let modified_string: String = temp_chars.iter().collect();
    deletion_set.insert(modified_string);
  }

  deletion_set
}

pub fn insertion(mut user_input: String) -> HashSet<String> {
  let mut insertion_set: HashSet<String> = HashSet::new();
  let string_length: usize = user_input.len();

  for c in 0..string_length+1 {
    for l in 'a'..='z' {
      user_input.insert(c, l);
      insertion_set.insert(user_input.clone());
      user_input.remove(c);
    }
  }

  insertion_set
}

pub fn substitution(mut user_input: String) -> HashSet<String> {
  let mut substitution_set: HashSet<String> = HashSet::new();
  let string_length: usize = user_input.len();

  for c in 0..string_length {
    let original_char: String = user_input[c..c+1].to_string();
    for l in 'a'..='z' {
      let mut buf: [u8; 4] = [0; 4];
      user_input.replace_range(c..c+1, l.encode_utf8(&mut buf));
      substitution_set.insert(user_input.clone());
      user_input.replace_range(c..c+1, &original_char);
    }
  }

  substitution_set
}

pub fn transposition(user_input: String) -> HashSet<String> {
  let mut transposition_set: HashSet<String> = HashSet::new();
  let string_length: usize = user_input.len();
  let mut string_vector: Vec<char> = user_input.chars().collect();

  for c in 0..string_length-1 {
    string_vector.swap(c, c+1);
    transposition_set.insert(string_vector.iter().collect());
    string_vector.swap(c, c+1);
  }

  transposition_set
}

pub fn search_dictionary(
  word_dictionary: &HashMap<String, u64>, 
  deletion_set: HashSet<String>, 
  insertion_set: HashSet<String>, 
  substitution_set: HashSet<String>, 
  transposition_set: HashSet<String>) 
  -> HashSet<String> {
  let mut in_dictionary: HashSet<String> = HashSet::new();
  let mut all_word_combinations: HashSet<String> = HashSet::new();

  let norvig_sets = vec![deletion_set, insertion_set, substitution_set, transposition_set];

  for set in norvig_sets {
    all_word_combinations.extend(set);
  }

  for item in all_word_combinations {
    if word_dictionary.contains_key(&item) {
      in_dictionary.insert(item);
    }
  }

  in_dictionary
}