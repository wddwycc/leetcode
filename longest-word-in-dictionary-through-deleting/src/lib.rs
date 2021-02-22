use std::{cmp::Reverse, str::Chars};

pub struct Solution;
impl Solution {
    // s.len() = n
    // d.len() = k
    // Time => O(nk + nlogn), space => O(1)
    pub fn find_longest_word(s: String, mut d: Vec<String>) -> String {
        d.sort_by(|a, b| (Reverse(a.len()), a).cmp(&(Reverse(b.len()), b)));
        for word in d {
            if Self::check_viable(word.chars(), s.chars()) {
                return word;
            }
        }
        return "".to_owned();
    }

    fn check_viable(word: Chars, mut src: Chars) -> bool {
        let mut word = word.peekable();
        while let Some(c) = word.peek() {
            loop {
                let sc = match src.next() {
                    Some(a) => a,
                    None => return false,
                };
                if &sc == c {
                    word.next();
                    break;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
