use std::collections::HashMap;

pub fn combine_frequency_score(
  in_dictionary: HashMap<String, f64>, 
  word_dictionary: &HashMap<String, u64>) -> Vec<(String, u64, f64)> {
  let mut in_dictionary_frequency_score: Vec<(String, u64, f64)> = Vec::new();

  for (word, edit_cost) in in_dictionary {
    if let Some(frequency) = word_dictionary.get(&word) {
      in_dictionary_frequency_score.push((word, *frequency, edit_cost))
    }
  }

  in_dictionary_frequency_score
}

pub fn score_ranking(in_dictionary_frequency_score: Vec<(String, u64, f64)>) -> Vec<(String, u64, f64)> {
  let mut frequency_vector: Vec<(String, u64, f64)> = in_dictionary_frequency_score.into_iter().collect();
  frequency_vector.sort_by(|a: &(String, u64, f64), b: &(String, u64, f64)| b.2.total_cmp(&a.2));

  frequency_vector.truncate(1);

  frequency_vector // returns the most frequent
}