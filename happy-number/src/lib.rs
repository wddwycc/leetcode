use std::collections::HashSet;

pub struct Solution {}
impl Solution {
    pub fn transform(n: i32) -> i32 {
        let mut n = n;
        let mut digits = vec![];
        while n > 0 {
            digits.push(n % 10);
            n = n / 10;
        }
        digits.iter().map(|a| a.pow(2)).sum()
    }

    pub fn is_happy(n: i32) -> bool {
        let mut visited = HashSet::new();
        let mut n = n;
        if n == 1 {
            return true;
        }
        loop {
            visited.insert(n);
            n = Solution::transform(n);
            if visited.contains(&n) {
                return false;
            }
            if n == 1 {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_happy(19), true);
        assert_eq!(Solution::is_happy(2), false);
    }
}
