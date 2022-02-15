use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut result = HashSet::new();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort_unstable();

    for &w in possible_anagrams.iter() {
        let w_lower = w.to_lowercase();
        if w_lower == word_lower {
            continue;
        }

        let mut anagram_chars: Vec<char> = w_lower.chars().collect();
        anagram_chars.sort_unstable();

        if anagram_chars == word_chars {
            result.insert(w);
        }
    }

    result
}
