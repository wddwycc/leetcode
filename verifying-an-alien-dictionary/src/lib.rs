use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let dict: HashMap<char, char> = order
            .chars()
            .zip("abcdefghijklmnopqrstuvwxyz".chars())
            .collect();
        let human_words: Vec<String> = words
            .iter()
            .map(|w| w.chars().map(|c| dict[&c]).collect())
            .collect();
        let mut sorted = human_words.clone();
        sorted.sort();
        sorted == human_words
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
