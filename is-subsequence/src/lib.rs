pub struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
            }
            j += 1;
        }
        i == s.len()
    }
}
