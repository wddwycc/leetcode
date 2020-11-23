pub struct Solution {}
impl Solution {
    // time: O(n)
    // space: O(1)
    pub fn reverse_words(s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut cur1 = 0;
        let mut cur2 = 0;
        loop {
            if chars[cur2] == ' ' || cur2 == chars.len() - 1 {
                // NOTE: do the swap
                let mut i = cur1;
                let mut j = {
                    if chars[cur2] == ' ' {
                        cur2 - 1
                    } else {
                        cur2
                    }
                };
                while i < j {
                    chars.swap(i, j);
                    i += 1;
                    j -= 1;
                }
                // NOTE: go to next word
                if cur2 < chars.len() - 1 {
                    cur1 = cur2 + 1;
                    cur2 = cur2 + 1;
                } else {
                    break;
                }
            } else {
                cur2 += 1;
            }
        }
        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_words(String::from("Let's take LeetCode contest")),
            String::from("s'teL ekat edoCteeL tsetnoc")
        );
    }
}
