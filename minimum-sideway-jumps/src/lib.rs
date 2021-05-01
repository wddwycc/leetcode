pub struct Solution;
impl Solution {
    pub fn min_side_jumps(mut obstacles: Vec<i32>) -> i32 {
        obstacles.reverse();
        let mut dp = (0, 0, 0);
        for i in 1..obstacles.len() {
            dp = match (obstacles[i - 1], obstacles[i]) {
                (0, 0) => (dp.0, dp.1, dp.2),

                (0, 1) | (1, 1) => (-1, dp.1, dp.2),
                (0, 2) | (2, 2) => (dp.0, -1, dp.2),
                (0, 3) | (3, 3) => (dp.0, dp.1, -1),

                (1, 0) => (std::cmp::min(dp.1, dp.2) + 1, dp.1, dp.2),
                (1, 2) => (dp.2 + 1, -1, dp.2),
                (1, 3) => (dp.1 + 1, dp.1, -1),
                (2, 0) => (dp.0, std::cmp::min(dp.0, dp.2) + 1, dp.2),
                (2, 1) => (-1, dp.2 + 1, dp.2),
                (2, 3) => (dp.0, dp.0 + 1, -1),
                (3, 0) => (dp.0, dp.1, (std::cmp::min(dp.0, dp.1)) + 1),
                (3, 1) => (-1, dp.1, dp.1 + 1),
                (3, 2) => (dp.0, -1, dp.0 + 1),
                _ => panic!(),
            }
        }
        dp.1
    }
}
