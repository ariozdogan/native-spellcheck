pub fn edit_score(in_dictionary_frequency_score: Vec<(String, u64, f64)>) -> Vec<(String, u64, f64)> {
    let mut scored_candidates: Vec<(String, u64, f64)> = Vec::new();

    for (word, frequency, edit_cost) in in_dictionary_frequency_score {
        let edit_score_value: f64 = frequency as f64 / edit_cost;
        scored_candidates.push((word, frequency, edit_score_value));
    }
    
    scored_candidates
}