pub struct Solution {}
impl Solution {
    /// time: O(n), space: O(1)
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        // f(1) = 1
        // f(2) = 2
        // f(n) = f(n - 1) + f(n - 2)
        let n = n as usize;
        let mut v1 = 1;
        let mut v2 = 2;
        for _ in 3..=n {
            let next_dp2 = v1 + v2;
            v1 = v2;
            v2 = next_dp2;
        }
        v2
    }
}

fn main() {
    println!("Hello, world!");
}
