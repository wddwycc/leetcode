use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut res: i32 = 0;
        let mut charts_map = HashMap::new();
        for char in chars.chars() {
            let entry = charts_map.entry(char).or_insert(0);
            *entry += 1;
        }
        for word in words {
            let mut map = charts_map.clone();
            for (idx, c) in word.chars().enumerate() {
                match map.get(&c) {
                    Some(a) => {
                        if *a > 0 {
                            map.entry(c).and_modify(|a| *a -= 1);
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
                if idx == word.len() - 1 {
                    res += word.len() as i32
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
    }
}
