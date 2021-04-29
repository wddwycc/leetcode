pub struct Solution;
impl Solution {
    pub fn min_available_duration(
        mut slots1: Vec<Vec<i32>>,
        slots2: Vec<Vec<i32>>,
        duration: i32,
    ) -> Vec<i32> {
        slots1.sort_by_key(|a| a[0]);
        for i in 0..slots1.len() {
            for j in 0..slots2.len() {
                let s1 = &slots1[i];
                let s2 = &slots2[j];
                let start = std::cmp::max(s1[0], s2[0]);
                let end = std::cmp::min(s1[1], s2[1]);
                if end - start >= duration {
                    return vec![start, start + duration];
                }
            }
        }
        vec![]
    }
}
