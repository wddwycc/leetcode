pub struct Solution;
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.into_bytes();
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] == s[j] {
                i += 1;
                j -= 1;
            } else {
                return Self::is_real_valid_palindrome(&s[(i + 1)..=j])
                    || Self::is_real_valid_palindrome(&s[i..=(j - 1)]);
            }
        }
        true
    }

    fn is_real_valid_palindrome(s: &[u8]) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] == s[j] {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
