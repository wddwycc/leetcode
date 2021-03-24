pub struct Solution;
impl Solution {
    // time: O(nlogn)
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        a.sort();
        let mut b: Vec<(usize, i32)> = b.into_iter().enumerate().collect();
        b.sort_by(|a, b| a.1.cmp(&b.1));

        let mut res = vec![0; n];
        let mut slow = 0;
        let mut fast = n - 1;
        while let Some((opponent_idx, opponent)) = b.pop() {
            res[opponent_idx] = {
                if opponent >= a[fast] {
                    let v = a[slow];
                    slow += 1;
                    v
                } else {
                    let v = a[fast];
                    fast -= 1;
                    v
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
            Solution::advantage_count(vec![9, 1, 2, 4, 5], vec![6, 2, 9, 1, 4]),
            [9, 4, 1, 2, 5]
        );
    }
}
