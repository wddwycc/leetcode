pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let s3 = s3.into_bytes();
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut cache = HashMap::new();
        Self::dfs(&s1, &s2, &s3, (0, 0), &mut cache)
    }

    fn dfs(
        s1: &[u8],
        s2: &[u8],
        s3: &[u8],
        cur: (usize, usize),
        cache: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(cached) = cache.get(&cur) {
            return *cached;
        }

        let (i, j) = cur;
        // case 1: reach s1 end;
        if i == s1.len() {
            let res = &s2[j..] == &s3[(i + j)..];
            cache.insert(cur, res);
            return res;
        }
        // case 2: reach s2 end;
        if j == s2.len() {
            let res = &s1[i..] == &s3[(i + j)..];
            cache.insert(cur, res);
            return res;
        }
        // case 3: s1[i] == s3[i + j], continue dfs
        if s1[i] == s3[i + j] && Self::dfs(s1, s2, s3, (i + 1, j), cache) {
            cache.insert(cur, true);
            return true;
        }
        // case 4: s2[j] == s3[i + j], continue dfs
        if s2[j] == s3[i + j] && Self::dfs(s1, s2, s3, (i, j + 1), cache) {
            cache.insert(cur, true);
            return true;
        }
        cache.insert(cur, false);
        return false;
    }
}
