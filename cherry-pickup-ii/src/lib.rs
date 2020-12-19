pub struct Solution;
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let r = grid.len();
        let c = grid[0].len();
        let mut dp = vec![vec![0; c + 2]; c + 2];
        for y in (1..=r).rev() {
            let mut tmp = vec![vec![0; c + 2]; c + 2];
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
                            tmp[x1][x2] = std::cmp::max(tmp[x1][x2], cur + dp[next_x1][next_x2])
                        }
                    }
                }
            }
            dp = tmp;
        }
        dp[1][c]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
