pub struct Solution;
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let n = data.len();
        let k = data.iter().filter(|&&a| a == 1).count();

        if k == 0 || k == 1 {
            return 0;
        }

        let mut first_is_zero = data[0] == 0;
        let mut steps = (0..k).filter(|&i| data[i] == 0).count() as i32;
        let mut min = steps;

        for i in 1..=(n - k) {
            if first_is_zero {
                steps -= 1;
            }
            if data[i + k - 1] == 0 {
                steps += 1;
            }
            first_is_zero = data[i] == 0;
            min = min.min(steps);
        }
        min
    }
}
