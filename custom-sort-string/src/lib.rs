pub struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let s = str.into_bytes();
        let mut buckets = vec![0; 26];
        for b in s {
            buckets[(b - b'a') as usize] += 1;
        }
        let mut ans = "".to_owned();
        for b in order.bytes() {
            let idx = (b - b'a') as usize;
            for _ in 0..buckets[idx] {
                ans.push(b as char);
            }
            buckets[idx] = 0;
        }
        for (idx, &size) in buckets.iter().enumerate() {
            let c = (b'a' + idx as u8) as char;
            for _ in 0..size {
                ans.push(c);
            }
        }
        ans
    }
}
