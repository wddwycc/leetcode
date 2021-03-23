use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let len = arr.len();
        if len < 3 {
            return 0;
        }

        let arr_dist = {
            let mut res = HashMap::new();
            for i in 0..len {
                *res.entry(arr[i]).or_insert(0) += 1;
            }
            res
        };
        let mut valid_nums: Vec<i32> = arr_dist.keys().map(|a| *a).collect();
        valid_nums.sort();

        let mut res = 0;

        for i in 0..valid_nums.len() {
            let num1 = valid_nums[i];
            for j in i..valid_nums.len() {
                let num2 = valid_nums[j];
                let num3 = target - num1 - num2;
                if num3 < num2 {
                    continue;
                }
                let dist = {
                    let mut res = HashMap::new();
                    for num in &[num1, num2, num3] {
                        *res.entry(*num).or_insert(0) += 1;
                    }
                    res
                };
                let count: i64 = dist
                    .iter()
                    .map(|(&num, &num_count)| {
                        let mut n = match arr_dist.get(&num) {
                            Some(a) => *a,
                            None => return 0,
                        };
                        if num_count > n {
                            return 0;
                        }
                        match num_count {
                            1 => n,
                            2 => n * (n - 1) / 2,
                            3 => {
                                let mut res = 0;
                                while n > 2 {
                                    res += (n - 1) * (n - 2) / 2;
                                    n -= 1;
                                }
                                res
                            }
                            _ => panic!(),
                        }
                    })
                    .product();
                res += count;
            }
        }

        let modulo = 10_i64.pow(9) + 7;
        (res % modulo) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8),
            20
        );
    }
}
