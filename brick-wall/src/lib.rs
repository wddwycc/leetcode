use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let h = wall.len();
        let mut map = HashMap::new();
        for row in wall {
            let mut acc = 0;
            for brick in &row[..row.len() - 1] {
                acc += brick;
                *map.entry(acc).or_insert(0) += 1;
            }
        }
        h as i32 - map.values().max().map(|a| *a).unwrap_or(0)
    }
}
