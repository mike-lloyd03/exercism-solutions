use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_words: HashMap<&&str, i32> = HashMap::new();

    magazine.iter().for_each(|w| {
        *mag_words.entry(w).or_default() += 1;
    });

    for word in note {
        let count = mag_words.entry(word).or_default();
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}
