pub struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res = 0;
        for b in n.into_bytes() {
            res = res.max(b - b'0');
        }
        res as i32
    }
}
