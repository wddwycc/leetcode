pub struct Solution;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s: Vec<char> = s.chars().collect();
        let n = s.len() - 1;
        let dict = HashSet::from_iter(word_dict);

        // given function f(i, j) -> bool, represents s[i..=j] is breakable by dict words
        // if f(i, j) = true, there must be a k (k >= i, k < j), satisfies:
        //   1. f(i, k) in dict
        //   2. f(k + 1, j) = true
        // when i == j, f(i, j) = true

        // let's try top-down dp
        let mut cache = HashMap::new();
        Self::dfs(&s, &dict, &mut cache, 0, n)
    }

    fn dfs(
        s: &[char],
        dict: &HashSet<String>,
        cache: &mut HashMap<(usize, usize), bool>,
        i: usize,
        j: usize,
    ) -> bool {
        if let Some(cached) = cache.get(&(i, j)) {
            return *cached;
        }

        if i > j {
            return true;
        }
        let res = {
            let mut res = false;
            let mut word = "".to_owned();
            for k in i..=j {
                word.push(s[k]);
                if dict.contains(&word) && Self::dfs(s, dict, cache, k + 1, j) {
                    res = true;
                    break;
                }
            }
            res
        };
        cache.insert((i, j), res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
