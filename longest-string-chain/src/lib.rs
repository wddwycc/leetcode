use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        // Group words by length
        let word_groups = {
            let mut res = vec![vec![]; 16];
            for word in words {
                let len = word.len();
                res[len - 1].push(word);
            }
            res
        };
        // DFS with memoization
        let mut res = 0;
        let mut cache = HashMap::new();
        for words in &word_groups {
            for word in words {
                res = res.max(Self::dfs(&word_groups, word.clone(), &mut cache));
            }
        }
        res
    }

    fn dfs(word_groups: &[Vec<String>], word: String, cache: &mut HashMap<String, i32>) -> i32 {
        if let Some(cached) = cache.get(&word) {
            return *cached;
        }

        let len = word.len();
        if len == 16 {
            return 1;
        }
        let mut res = 1;
        for next_word in &word_groups[len] {
            if Self::is_predecessor(word.as_bytes(), next_word.as_bytes()) {
                res = res.max(Self::dfs(word_groups, next_word.clone(), cache) + 1)
            }
        }
        cache.insert(word, res);
        res
    }

    fn is_predecessor(word1: &[u8], word2: &[u8]) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < word1.len() {
            if i + 1 < j {
                return false;
            }
            if word1[i] == word2[j] {
                i += 1;
                j += 1;
                continue;
            } else {
                j += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = vec![
            "ksqvsyq",
            "ks",
            "kss",
            "czvh",
            "zczpzvdhx",
            "zczpzvh",
            "zczpzvhx",
            "zcpzvh",
            "zczvh",
            "gr",
            "grukmj",
            "ksqvsq",
            "gruj",
            "kssq",
            "ksqsq",
            "grukkmj",
            "grukj",
            "zczpzfvdhx",
            "gru",
        ]
        .into_iter()
        .map(|a| a.to_owned())
        .collect();
        let res = Solution::longest_str_chain(words);
        assert_eq!(res, 7);
    }
}
