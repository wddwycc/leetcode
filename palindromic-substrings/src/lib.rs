pub struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut res = 0;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..(chars.len() as i32) {
            res += Self::count_for_center(&chars, i, i);
            res += Self::count_for_center(&chars, i, i + 1);
        }
        res
    }

    fn count_for_center(chars: &[char], mut i: i32, mut j: i32) -> i32 {
        let mut res = 0;
        while i >= 0 && j < chars.len() as i32 {
            if chars[i as usize] == chars[j as usize] {
                res += 1;
            } else {
                break;
            }
            i -= 1;
            j += 1;
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
