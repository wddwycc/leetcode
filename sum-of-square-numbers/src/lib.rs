pub struct Solution;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let mut i = 0;
        let mut j = (c as f64).sqrt().floor() as i32;
        while i <= j {
            let sum = i * i + j * j;
            if sum == c {
                return true;
            } else if sum > c {
                j -= 1;
            } else {
                i += 1;
            }
        }
        false
    }
}
