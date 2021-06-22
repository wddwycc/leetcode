use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut words = {
            let mut res = HashMap::new();
            for word in words {
                let (head, word) = word.split_at(1);
                let head = head.chars().next().unwrap();
                res.entry(head).or_insert(vec![]).push(word.to_string());
            }
            res
        };
        let mut ans = 0;
        for c in s.chars() {
            let matched_words = match words.remove(&c) {
                Some(a) => a,
                None => continue,
            };
            for word in matched_words {
                if word.len() == 0 {
                    ans += 1;
                    continue;
                }
                let (head, word) = word.split_at(1);
                let head = head.chars().next().unwrap();
                words.entry(head).or_insert(vec![]).push(word.to_string());
            }
        }
        ans
    }
}
