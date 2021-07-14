pub struct Solution;
impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let s = str.into_bytes();
        let mut buckets = vec![0; 26];
        for b in s {
            buckets[(b - b'a') as usize] += 1;
        }
        let mut ans = "".to_owned();
        for c in order.chars() {
            let idx = (c as u8 - 'a' as u8) as usize;
            for _ in 0..buckets[idx] {
                ans.push(c);
            }
            buckets[idx] = 0;
        }
        for (idx, &size) in buckets.iter().enumerate() {
            let c = ('a' as u8 + idx as u8) as char;
            for _ in 0..size {
                ans.push(c);
            }
        }
        ans
    }
}
