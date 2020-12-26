pub struct Solution;
// time: O(n), space: O(1)
impl Solution {
    fn helper(src: &[char], start: usize, end: usize) -> bool {
        let mut i = start;
        let mut j = end;
        while i < j {
            if src[i] == src[j] {
                i += 1;
                j -= 1;
            } else {
                return false;
            }
        }
        true
    }

    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        if chars.len() == 0 {
            return true;
        }
        Self::helper(&chars, 0, chars.len() - 1)
    }
}

fn main() {
    assert_eq!(
        Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
        true
    );
}
