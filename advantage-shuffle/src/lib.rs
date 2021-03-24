pub struct Solution;
impl Solution {
    // time: O(n^2)
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        a.sort();
        let mut b: Vec<(usize, i32)> = b.into_iter().enumerate().collect();
        b.sort_by(|a, b| a.1.cmp(&b.1));
        let (_, offset) = Self::best_solution(&a, &b.iter().map(|a| a.1).collect::<Vec<i32>>(), 0);
        for i in 0..offset {
            let fst = a.remove(0);
            a.insert(n - i - 1, fst);
        }

        let mut res = vec![0; n];
        for i in 0..n {
            res[b[i].0] = a[i];
        }
        res
    }

    // find best offset to apply to reach max advantage for two sorted array
    fn best_solution(a: &[i32], b: &[i32], offset: usize) -> (usize, usize) {
        let n = a.len();
        if n == 1 {
            if a[0] > b[0] {
                return (1, offset);
            } else {
                return (0, offset);
            }
        }
        let res: usize = (0..n).map(|i| if a[i] > b[i] { 1 } else { 0 }).sum();
        if a[0] < b[n - 1] {
            let new_res = Self::best_solution(&a[1..], &b[..(n - 1)], offset + 1);
            if new_res.0 > res {
                return new_res;
            }
        }
        (res, offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::advantage_count(vec![9, 1, 2, 4, 5], vec![6, 2, 9, 1, 4]),
            [9, 4, 1, 2, 5]
        );
    }
}
