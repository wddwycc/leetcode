use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut words = {
            let mut res = HashMap::new();
            for mut word in words {
                let head = word.remove(0);
                res.entry(head).or_insert(vec![]).push(word);
            }
            res
        };
        let mut ans = 0;
        for c in s.chars() {
            let matched_words = match words.remove(&c) {
                Some(a) => a,
                None => continue,
            };
            for mut word in matched_words {
                if word.len() == 0 {
                    ans += 1;
                    continue;
                }
                let head = word.remove(0);
                words.entry(head).or_insert(vec![]).push(word);
            }
        }
        ans
    }
}
