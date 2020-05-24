pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut ways: Vec<i32> = vec![];
        for i in 1..=n {
            match i {
                1 => ways.push(1),
                2 => ways.push(2),
                _ => ways.push(ways[i as usize - 2] + ways[i as usize - 3]),
            }
        }
        *ways.last().unwrap_or(&0)
    }
}

fn main() {
    println!("Hello, world!");
}
