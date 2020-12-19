pub struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut dp = vec![vec![vec![0; c + 2]; c + 2]; r + 2];
        for y in (1..=r).rev() {
            for x1 in 1..=c {
                for x2 in 1..=c {
                    let cur = grid[y - 1][x1 - 1] + {
                        if x1 != x2 {
                            grid[y - 1][x2 - 1]
                        } else {
                            0
                        }
                    };
                    for next_x1 in (x1 - 1)..=(x1 + 1) {
                        for next_x2 in (x2 - 1)..=(x2 + 1) {
                            dp[y][x1][x2] =
                                std::cmp::max(dp[y][x1][x2], cur + dp[y + 1][next_x1][next_x2])
                        }
                    }
                }
            }
        }
        dp[1][1][c]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
