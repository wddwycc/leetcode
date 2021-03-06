pub struct Solution;
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        // find all 01 and 10, extend left and right
        let bytes = s.into_bytes();
        let mut res = 0;
        for cur in 0..(bytes.len() - 1) {
            let l = bytes[cur];
            let r = bytes[cur + 1];
            if l == r {
                continue;
            }
            res += 1;
            let mut i = cur;
            let mut j = cur + 1;
            while i > 0 && j + 1 < bytes.len() {
                i -= 1;
                j += 1;
                if bytes[i] == l && bytes[j] == r {
                    res += 1;
                } else {
                    break;
                }
            }
        }
        res
    }
}
