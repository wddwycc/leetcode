pub struct Solution;
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        Self::calc(&chars, 0, chars.len() - 1)
    }

    fn calc(chars: &[char], l: usize, r: usize) -> i32 {
        if l + 1 == r {
            return 1;
        }

        let mut l_bound = l;
        let mut acc = 0;
        let mut sum = 0;
        let mut partitions = 0;
        for idx in l..=r {
            let c = chars[idx];
            if c == '(' {
                acc += 1;
            }
            if c == ')' {
                acc -= 1;
            }
            // if is single partition, trigger nested calc
            if idx == r && partitions == 0 {
                return Self::calc(chars, l + 1, r - 1) * 2;
            }
            if acc == 0 {
                partitions += 1;
                sum += Self::calc(chars, l_bound, idx);
                l_bound = idx + 1;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
