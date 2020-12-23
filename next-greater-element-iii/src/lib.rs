pub struct Solution;
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = Self::to_digits(n);
        let i = match (0..(digits.len() - 1)).find(|a| digits[a + 1] < digits[*a]) {
            Some(a) => a,
            None => return -1,
        };
        let j = {
            let mut res = None;
            for n in 0..=i {
                if digits[i + 1] < digits[n] {
                    if let Some(prev_res) = res {
                        if digits[n] < digits[prev_res] {
                            res = Some(n)
                        }
                    } else {
                        res = Some(n)
                    }
                }
            }
            match res {
                Some(a) => a,
                None => return -1,
            }
        };
        digits.swap(i + 1, j);
        digits[0..=i].sort_by(|a, b| b.cmp(a));
        Self::to_num(&digits)
    }

    // MARK: Utils

    fn to_digits(mut src: i32) -> Vec<i32> {
        let mut res = vec![];
        while src > 0 {
            res.push(src % 10);
            src /= 10;
        }
        res
    }

    fn to_num(src: &[i32]) -> i32 {
        let mut res = 0;
        let mut base = 1;
        let max_base = (10 as i32).pow(9);
        let max_base_n = 2;
        for i in src {
            // check boundary
            if base == max_base {
                if i == &max_base_n && std::i32::MAX - max_base * max_base_n < res {
                    return -1;
                }
                if i > &max_base_n {
                    return -1;
                }
            }
            res += i * base;
            base *= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::next_greater_element(2147483647), -1);
    }
}
