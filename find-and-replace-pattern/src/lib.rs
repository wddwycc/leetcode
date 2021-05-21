use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let pattern = Self::pattern(&pattern);
        words
            .into_iter()
            .filter(|src| Self::pattern(src) == pattern)
            .collect()
    }

    fn pattern(src: &String) -> Vec<Vec<usize>> {
        let mut res: HashMap<char, Vec<usize>> = HashMap::new();
        for (idx, c) in src.chars().enumerate() {
            res.entry(c).or_insert(vec![]).push(idx);
        }
        let mut res: Vec<Vec<usize>> = res.into_iter().map(|(_, v)| v).collect();
        res.sort();
        res
    }
}
