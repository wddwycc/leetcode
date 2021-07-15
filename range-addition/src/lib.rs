pub struct Solution;
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let n = length as usize;
        let mut res = vec![0; n];
        for a in updates {
            let start = a[0] as usize;
            let end = a[1] as usize;
            let offset = a[2];
            res[start] += offset;
            if end + 1 < n {
                res[end + 1] -= offset;
            }
        }
        for i in 1..n {
            res[i] = res[i - 1] + res[i];
        }
        res
    }
}
