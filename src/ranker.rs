use std::collections::{HashSet, HashMap};
use std::cmp::Reverse;

pub fn in_dictionary_frequency(
  in_dictionary: HashSet<String>, word_dictionary: &HashMap<String, u64>) -> HashMap<String, u64> {
  let mut word_frequency: HashMap<String, u64> = HashMap::new();

  for item in in_dictionary {
    if let Some(frequency) = word_dictionary.get(&item) {
      word_frequency.insert(item, *frequency);
    }
  }

  word_frequency
}

pub fn frequency_ranking(word_frequency: HashMap<String, u64>) -> Vec<(String, u64)> {
  let mut frequency_vector: Vec<(String, u64)> = word_frequency.into_iter().collect();
  frequency_vector.sort_by_key(|k: &(String, u64)| Reverse(k.1));

  frequency_vector.truncate(5);

  frequency_vector // returns top 5 most frequent
}