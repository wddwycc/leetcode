use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let bytes = word.into_bytes();
        // NOTE: Find all a starts
        let a_indice = {
            let mut res = vec![];
            for i in 0..bytes.len() {
                if bytes[i] == b'a' && (i == 0 || bytes[i - 1] != b'a') {
                    res.push(i);
                }
            }
            res
        };
        let mut res = 0;
        for l in a_indice {
            let mut r = l;
            let mut appeared = HashSet::new();
            appeared.insert(bytes[r]);
            while r + 1 < bytes.len() {
                if bytes[r + 1] < bytes[r] {
                    if appeared.len() == 5 {
                        res = res.max(r - l + 1);
                    }
                    break;
                } else {
                    r += 1;
                    appeared.insert(bytes[r]);
                }
            }
            if appeared.len() == 5 {
                res = res.max(r - l + 1);
            }
        }
        res as i32
    }
}
