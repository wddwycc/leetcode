pub struct Solution;
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let words: Vec<(u32, usize)> = words
            .into_iter()
            .map(|word| {
                let len = word.len();
                let mut bitmask = 0;
                for b in word.into_bytes() {
                    bitmask |= 1 << (b - b'a' + 1);
                }
                (bitmask, len)
            })
            .collect();
        let mut res = 0;
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                let &(b_l, len_l) = &words[i];
                let &(b_r, len_r) = &words[j];
                if b_l & b_r == 0 {
                    res = res.max(len_l * len_r);
                }
            }
        }
        res as i32
    }
}
