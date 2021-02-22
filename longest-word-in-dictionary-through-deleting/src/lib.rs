use std::cmp::Reverse;

pub struct Solution;
impl Solution {
    // s.len() = n
    // d.len() = k
    // Time => O(nk + nlogn), space => O(1)
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        d.sort_by(|a, b| (Reverse(a.len()), a).cmp(&(Reverse(b.len()), b)));

        let s: Vec<char> = s.chars().collect();
        for word in d {
            if Self::check_viable(&word.chars().collect::<Vec<char>>(), &s) {
                return word;
            }
        }
        return "".to_owned();
    }

    fn check_viable(word: &[char], src: &[char]) -> bool {
        let m = word.len();
        let n = src.len();

        let mut i = 0;
        let mut j = 0;
        while i < m && j < n {
            if word[i] == src[j] {
                i += 1;
            }
            j += 1;
        }
        i == m
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
