use std::collections::HashMap;

pub struct Solution;
impl Solution {
    // Time: O(n), Space: O(1)
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let keyboard = {
            let mut res = HashMap::new();
            for (i, c) in keyboard.char_indices() {
                res.insert(c, i as i32);
            }
            res
        };

        let mut cur = 0;
        let mut res = 0;
        for c in word.chars() {
            let next_cur = *keyboard.get(&c).unwrap();
            res += (next_cur - cur).abs();
            cur = next_cur;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
