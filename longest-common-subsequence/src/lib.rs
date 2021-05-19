use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.into_bytes();
        let text2 = text2.into_bytes();

        let mut cache = HashMap::new();
        Self::dfs(&text1, &text2, 0, 0, &mut cache)
    }

    pub fn dfs(
        text1: &[u8],
        text2: &[u8],
        cur1: usize,
        cur2: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if cur1 == text1.len() || cur2 == text2.len() {
            return 0;
        }
        if let Some(cached) = cache.get(&(cur1, cur2)) {
            return *cached;
        }
        // opt1: skip cur1.
        let mut res = Self::dfs(text1, text2, cur1 + 1, cur2, cache);
        // opt2: match cur1 in text2, then match the rest
        if let Some(opt) = text2
            .iter()
            .enumerate()
            .skip(cur2)
            .find(|(_, &a)| a == text1[cur1])
            .map(|(idx, _)| idx)
            .map(|idx| Self::dfs(text1, text2, cur1 + 1, idx + 1, cache) + 1)
        {
            res = res.max(opt);
        }
        cache.insert((cur1, cur2), res);
        res
    }
}
